mod external_links;
mod fetcher;
mod sheet;
mod sst;
mod styles;
mod utils;
mod vertex;

use logisheets_base::id_fetcher::SheetIdFetcherTrait;
use logisheets_workbook::prelude::*;
use sheet::load_comments;

use crate::{
    controller::{status::Status, Controller},
    file_loader2::{
        external_links::load_external_link,
        sheet::{load_cols, load_merge_cells, load_sheet_data, load_sheet_format_pr},
        styles::StyleLoader,
    },
    id_manager::SheetIdManager,
    settings::Settings,
};
pub struct SheetIdFetcher<'a> {
    pub sheet_id_manager: &'a mut SheetIdManager,
}

impl<'a> SheetIdFetcherTrait for SheetIdFetcher<'a> {
    fn fetch_sheet_id(&mut self, sheet_name: &str) -> logisheets_base::SheetId {
        self.sheet_id_manager.get_id(sheet_name)
    }
}

pub fn load(wb: Workbook, book_name: String) -> Controller {
    let Status {
        mut navigator,
        mut vertex_manager,
        mut container,
        mut sheet_id_manager,
        mut func_id_manager,
        mut text_id_manager,
        mut name_id_manager,
        mut external_links_manager,
        mut sheet_pos_manager,
        mut style_manager,
        mut cell_attachment_manager,
    } = Status::default();
    let mut sheet_id_fetcher = SheetIdFetcher {
        sheet_id_manager: &mut sheet_id_manager,
    };
    if let Some(ers) = &wb.workbook_part.external_references {
        ers.external_references.iter().for_each(|er| {
            let id = &er.id;
            if let Some(link) = wb.external_links.get(id) {
                load_external_link(&mut external_links_manager, link, &mut sheet_id_fetcher);
            }
        })
    }
    let mut settings = Settings::default();
    if let Some(calc_pr) = &wb.workbook_part.calc_pr {
        settings.calc_config.iter_limit = calc_pr.iterate_count as u16;
        settings.calc_config.error = calc_pr.iterate_delta as f32;
    }
    let mut style_loader = StyleLoader::new(&mut style_manager, &wb.styles);
    wb.workbook_part.sheets.sheets.iter().for_each(|ct_sheet| {
        let sheet_name = &ct_sheet.name;
        let sheet_id = sheet_id_manager.get_id(sheet_name);
        sheet_pos_manager.pos.push_back(sheet_id);
        if ct_sheet.state != StSheetState::Visible {
            sheet_pos_manager.hiddens.insert(sheet_id);
        }
        let id = &ct_sheet.id;
        if let Some(ws) = wb.worksheets.get(id) {
            if let Some(comments) = &ws.comments {
                load_comments(
                    sheet_id,
                    comments,
                    &mut navigator,
                    &mut cell_attachment_manager,
                );
            }
            if let Some(cols) = &ws.worksheet_part.cols {
                load_cols(
                    sheet_id,
                    &cols.cols,
                    &mut container,
                    &mut style_loader,
                    &mut navigator,
                )
            }
            if let Some(merge_cells) = &ws.worksheet_part.merge_cells {
                load_merge_cells(
                    sheet_id,
                    merge_cells,
                    &mut navigator,
                    &mut cell_attachment_manager,
                )
            }
            if let Some(sheet_format_pr) = &ws.worksheet_part.sheet_format_pr {
                load_sheet_format_pr(&mut settings, sheet_id, sheet_format_pr)
            }
            load_sheet_data(
                sheet_id,
                &book_name,
                &ws.worksheet_part.sheet_data,
                &mut navigator,
                &mut sheet_id_manager,
                &mut text_id_manager,
                &mut func_id_manager,
                &mut name_id_manager,
                &mut external_links_manager,
                &mut container,
                &mut vertex_manager,
                &mut style_loader,
                &wb,
            )
        }
    });
    let status = Status {
        navigator,
        vertex_manager,
        container,
        sheet_id_manager,
        func_id_manager,
        text_id_manager,
        name_id_manager,
        external_links_manager,
        sheet_pos_manager,
        style_manager,
        cell_attachment_manager,
    };
    Controller::from(status, book_name, settings)
}
