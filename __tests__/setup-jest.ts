import Adapter from '@wojtekmaj/enzyme-adapter-react-17'
import {configure} from 'enzyme'
import '@testing-library/jest-dom'
import { toHaveNoViolations } from 'jest-axe'

configure({adapter: new Adapter()})
expect.extend(toHaveNoViolations)
