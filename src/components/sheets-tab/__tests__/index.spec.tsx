/**
 * @jest-environment jsdom
 */
import {SheetsTabComponent} from '../index'
import {mount} from 'enzyme'
import {render} from '@testing-library/react'

describe('SheetTabComponent', () => {
    it.skip('base test', () => {
        const wrapper = render(<SheetsTabComponent></SheetsTabComponent>)
        expect(wrapper).toMatchSnapshot()
    })
    it('Should add tab', () => {
        jest.mock('@/core/ioc/provider', () => {
            return {
                useInjection: jest.fn().mockReturnValue({
                    getSheets: () => (['foo', 'bar'])
                }),
            }
        })
        const wrapper = mount(<SheetsTabComponent></SheetsTabComponent>)
    })
})
