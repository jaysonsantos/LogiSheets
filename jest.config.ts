import type { Config } from '@jest/types'

const config: Config.InitialOptions = {
    preset: 'ts-jest',
    verbose: true,
    silent: false,
    cache: true,
    rootDir: '.',
    testEnvironment: 'jsdom',
    // collectCoverage: true,
    moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx'],
    setupFilesAfterEnv: ['<rootDir>/__tests__/setup-jest.ts'],
    testMatch: [
        '**/?(*.)+(spec|test).[jt]s?(x)',
    ],
    moduleNameMapper: {
        '@/(.*)': '<rootDir>/src/$1',
        // '\\.(css|less|scss)$': 'identity-obj-proxy',
        '/\\.(css|less|scss)$/': 'identity-obj-proxy',
        'wasms/(server)/pkg$': '<rootDir>/__tests__/__mocks__/wasmMock.ts',
    },
    modulePaths: [
        '<rootDir>'
    ],
    globals: {
        'ts-jest': {
            // diagnostics: false,
            tsconfig: '<rootDir>/__tests__/tsconfig.json',
        }
    },
    testEnvironmentOptions: {
        url: 'http://localhost',
    },
}

export default config