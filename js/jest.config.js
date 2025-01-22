/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
    preset: 'ts-jest',
    testEnvironment: 'node',
    roots: ['<rootDir>/src', '<rootDir>/test'],
    testMatch: [
      '**/__tests__/**/*.+(ts|tsx|js)',
      '**/?(*.)+(spec|test).+(ts|tsx|js)'
    ],
    transform: {
      '^.+\\.(ts|tsx)$': 'ts-jest'
    },
    moduleNameMapper: {
      '^@/(.*)$': '<rootDir>/src/$1'
    },
    collectCoverage: true,
    coverageDirectory: 'coverage',
    coverageReporters: ['text', 'lcov', 'clover'],
    coverageThreshold: {
      global: {
        branches: 80,
        functions: 80,
        lines: 80,
        statements: 80
      }
    },
    verbose: true,
    testTimeout: 30000,
    setupFilesAfterEnv: ['<rootDir>/test/setup.ts'],
    globals: {
      'ts-jest': {
        tsconfig: 'tsconfig.json',
        diagnostics: {
          warnOnly: true
        }
      }
    },
    moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx', 'json', 'node'],
    testPathIgnorePatterns: [
      '/node_modules/',
      '/dist/'
    ],
    clearMocks: true,
    resetMocks: false,
    restoreMocks: true,
    errorOnDeprecated: true,
    maxWorkers: '50%',
    watchPlugins: [
      'jest-watch-typeahead/filename',
      'jest-watch-typeahead/testname'
    ]
  }