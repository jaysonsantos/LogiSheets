use logisheets_base::{CellId, CellValue, SheetId};
use logisheets_workbook::prelude::*;

use crate::{
    cell::Cell,
    cell_attachments::{comment::Comment, CellAttachmentsManager},
    container::{col_info_manager::ColInfo, row_info_manager::RowInfo, DataContainer},
    ext_book_manager::ExtBooksManager,
    id_manager::{FuncIdManager, NameIdManager, SheetIdManager, TextIdManager},
    navigator::Navigator,
    settings::Settings,
    vertex_manager::VertexManager,
};

use super::{
    fetcher::Fetcher,
    styles::StyleLoader,
    utils::{parse_cell, parse_range},
    vertex::{load_normal_formula, load_shared_formulas},
};

pub fn load_cols(
    sheet_id: SheetId,
    cols: &Vec<CtCol>,
    container: &mut DataContainer,
    style_loader: &mut StyleLoader,
    navigator: &mut Navigator,
) {
    cols.iter().for_each(|col| {
        let min = col.min - 1;
        let max = col.max - 1;
        let col_style_id = style_loader.load_xf(col.style);
        (min..max + 1).into_iter().for_each(|col_idx| {
            let col_id = navigator
                .fetch_col_id(sheet_id, col_idx as usize)
                .unwrap_or(0);
            let col_info = ColInfo {
                best_fit: col.best_fit,
                collapsed: col.collapsed,
                custom_width: col.custom_width,
                hidden: col.hidden,
                outline_level: col.outline_level as u8,
                style: col_style_id,
                width: col.width,
            };
            container.set_col_info(sheet_id, col_id, col_info);
        });
    });
}

pub fn load_merge_cells(
    sheet_id: SheetId,
    merge_cells: &CtMergeCells,
    navigator: &mut Navigator,
    cell_attachment_manager: &mut CellAttachmentsManager,
) {
    merge_cells.merge_cells.iter().for_each(|mc| {
        let r = &mc.reference;
        if let Some(((start_row, start_col), (end_row, end_col))) = parse_range(&r) {
            let start_id = navigator.fetch_cell_id(sheet_id, start_row, start_col);
            let end_id = navigator.fetch_cell_id(sheet_id, end_row, end_col);
            match (start_id, end_id) {
                (Some(start), Some(end)) => match (start, end) {
                    (CellId::NormalCell(s), CellId::NormalCell(e)) => {
                        cell_attachment_manager
                            .merge_cells
                            .add_merge_cell2(sheet_id, s, e);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    })
}

pub fn load_comments(
    sheet_id: SheetId,
    comments: &Comments,
    navigator: &mut Navigator,
    cell_attachment_manager: &mut CellAttachmentsManager,
) {
    let authors = comments
        .authors
        .authors
        .iter()
        .map(|plain_text| plain_text.value.to_string())
        .collect::<Vec<_>>();
    comments
        .comment_list
        .comments
        .iter()
        .for_each(|c| match parse_cell(&c.reference) {
            Some((row, col)) => {
                if let Some(cell_id) = navigator.fetch_cell_id(sheet_id, row, col) {
                    let text = rst_to_plain_text(&c.text);
                    let author = authors.get(c.author_id as usize).unwrap();
                    let author_id = cell_attachment_manager.comments.authors.get_id(author);
                    let comment = Comment {
                        author: author_id,
                        text,
                    };
                    cell_attachment_manager
                        .comments
                        .add_comment(sheet_id, cell_id, comment);
                }
            }
            None => {}
        })
}

pub fn load_sheet_data(
    sheet_id: SheetId,
    book_name: &str,
    sheet_data: &CtSheetData,
    navigator: &mut Navigator,
    sheet_id_manager: &mut SheetIdManager,
    text_id_manager: &mut TextIdManager,
    func_id_manager: &mut FuncIdManager,
    name_id_manager: &mut NameIdManager,
    ext_books_manager: &mut ExtBooksManager,
    container: &mut DataContainer,
    vertex_manager: &mut VertexManager,
    style_loader: &mut StyleLoader,
    workbook: &Workbook,
) {
    sheet_data.rows.iter().for_each(|row| {
        let style_id = style_loader.load_xf(row.s);
        if let Some(idx) = row.r {
            if idx >= 1 {
                let row_info = RowInfo {
                    collapsed: row.collapsed,
                    custom_format: row.custom_format,
                    hidden: row.hidden,
                    ht: row.ht,
                    outline_level: row.outline_level,
                    style: style_id,
                };
                let id = navigator.fetch_row_id(sheet_id, idx as usize - 1).unwrap();
                container.set_row_info(sheet_id, id, row_info);
            }
        }
        row.cells.iter().for_each(|ct_cell| {
            if let Some(r) = &ct_cell.r {
                if let Some((row, col)) = parse_cell(r) {
                    let cv = CellValue::from_cell(ct_cell, |idx| {
                        let rst = workbook.sst.as_ref().unwrap().si.get(idx).unwrap();
                        let string = rst_to_plain_text(rst);
                        text_id_manager.get_id(&string)
                    });
                    let id = navigator.fetch_cell_id(sheet_id, row, col).unwrap();
                    let style_id = style_loader.load_xf(ct_cell.s);
                    let cell = Cell {
                        value: cv,
                        style: style_id,
                    };
                    container.add_cell(sheet_id, id, cell);
                    if let Some(formula) = &ct_cell.f {
                        let mut fetcher = Fetcher {
                            sheet_id_manager,
                            text_id_manager,
                            func_id_manager,
                            name_id_manager,
                            navigator,
                            ext_books_manager,
                            workbook,
                        };
                        if let Some(f) = &formula.formula {
                            if let Some(reference) = &formula.reference {
                                if let Some(((row_start, col_start), (row_end, col_end))) =
                                    parse_range(reference)
                                {
                                    load_shared_formulas(
                                        vertex_manager,
                                        book_name,
                                        sheet_id,
                                        row_start,
                                        col_start,
                                        row_start,
                                        col_start,
                                        row_end,
                                        col_end,
                                        f,
                                        &mut fetcher,
                                    )
                                } else if let Some((row_idx, col_idx)) = parse_cell(reference) {
                                    load_normal_formula(
                                        vertex_manager,
                                        book_name,
                                        sheet_id,
                                        row_idx,
                                        col_idx,
                                        f,
                                        &mut fetcher,
                                    )
                                }
                            } else {
                                load_normal_formula(
                                    vertex_manager,
                                    book_name,
                                    sheet_id,
                                    row,
                                    col,
                                    f,
                                    &mut fetcher,
                                )
                            }
                        }
                    }
                }
            }
        })
    });
}

pub fn load_sheet_format_pr(
    settings: &mut Settings,
    sheet_id: SheetId,
    sheet_format_pr: &CtSheetFormatPr,
) {
    settings
        .sheet_format_pr
        .insert(sheet_id, sheet_format_pr.clone());
}

fn rst_to_plain_text(rst: &CtRst) -> String {
    match &rst.t {
        Some(p) => p.value.to_string(),
        None => {
            let mut result = String::from("");
            rst.r.iter().for_each(|relt| {
                let s = relt.t.value.to_string();
                result.push_str(s.as_str());
            });
            result
        }
    }
}
