const path = require('node:path');

const { rootNodeFromAnchor } = require('@codama/nodes-from-anchor');
const { readJson } = require('@codama/renderers-core');
const { visit } = require('@codama/visitors-core');

const { renderVisitor: renderVixenVisitor } = require('../../dist/index.node.cjs');
const { renderVisitor: renderRustVisitor } = require('@codama/renderers-rust');

function generateProject(project, node, generateProto) {
    const crateFolder = __dirname;

    const definedTypes = node.program.definedTypes;
    // Push `fees` defined type (not included in idl)
    // // https://github.com/raydium-io/raydium-amm/blob/master/program/src/state.rs#L475-L496
    definedTypes.push({
        kind: 'definedTypeNode',
        name: 'fees',
        docs: [],
        type: {
            kind: 'structTypeNode',
            fields: [
                'minSeparateNumerator',
                'minSeparateDenominator',
                'tradeFeeNumerator',
                'tradeFeeDenominator',
                'pnlNumerator',
                'pnlDenominator',
                'swapFeeNumerator',
                'swapFeeDenominator',
            ].map(fieldName => ({
                kind: 'structFieldTypeNode',
                name: fieldName,
                docs: [],
                type: { kind: 'numberTypeNode', format: 'u64', endian: 'le' },
            })),
        },
    });

    const accounts = node.program.accounts;
    // Add `ammConfig` account (not included in idl)
    // https://github.com/raydium-io/raydium-amm/blob/master/program/src/state.rs#L860-L871
    const ammConfig = {
        kind: 'accountNode',
        name: 'ammConfig',
        docs: [],
        data: {
            kind: 'structTypeNode',
            fields: [
                {
                    kind: 'structFieldTypeNode',
                    name: 'pnlOwner',
                    docs: [],
                    type: { kind: 'publicKeyTypeNode' },
                },
                {
                    kind: 'structFieldTypeNode',
                    name: 'cancelOwner',
                    docs: [],
                    type: { kind: 'publicKeyTypeNode' },
                },
                {
                    kind: 'structFieldTypeNode',
                    name: 'pending1',
                    docs: [],
                    type: {
                        kind: 'arrayTypeNode',
                        item: { kind: 'numberTypeNode', format: 'u64', endian: 'le' },
                        count: { kind: 'fixedCountNode', value: 28 },
                    },
                },
                {
                    kind: 'structFieldTypeNode',
                    name: 'pending2',
                    docs: [],
                    type: {
                        kind: 'arrayTypeNode',
                        item: { kind: 'numberTypeNode', format: 'u64', endian: 'le' },
                        count: { kind: 'fixedCountNode', value: 31 },
                    },
                },
                {
                    kind: 'structFieldTypeNode',
                    name: 'createPoolFee',
                    docs: [],
                    type: { kind: 'numberTypeNode', format: 'u64', endian: 'le' },
                },
            ],
        },
        discriminators: [],
    };
    accounts.push(ammConfig);

    const ammConfigIndex = definedTypes.findIndex(definedType => definedType.name === 'ammConfig');
    definedTypes.splice(ammConfigIndex, 1);
    const feesIndex = accounts.findIndex(account => account.name === 'fees');
    accounts.splice(feesIndex, 1);

    const updatedNode = { ...node, program: { ...node.program, definedTypes, accounts } };

    // #Renderers-rust
    visit(
        updatedNode,
        renderRustVisitor(path.join(crateFolder, 'src', 'generated_sdk'), {
            crateFolder,
            formatCode: true,
        }),
    );

    //  #Render Vixen Parser
    visit(
        updatedNode,
        renderVixenVisitor(crateFolder, {
            sdkName: 'crate',
            crateFolder,
            formatCode: true,
            generateProto,
            project,
            generatedFolderName: 'generated_parser',
            deleteFolderBeforeRendering: false,
            cargoAdditionalDependencies: [
                'yellowstone-vixen-core = { git = "https://github.com/rpcpool/yellowstone-vixen", branch = "main",features = ["proto"] }',
                'num-derive = "0.4"',
                'thiserror = "1.0.64"',
                'num-traits = "^0.2"',
                'tracing = { version = "0.1.40", optional = true }',
                'strum = { version = "0.24", optional = true }',
                'strum_macros = { version = "0.24", optional = true }',
                '\n',
                '[features]',
                'anchor = []',
                'anchor-idl-build = []',
                'serde = []',
                'test-sbf = []',
                'fetch = []',
                'tracing = ["dep:tracing", "dep:strum", "dep:strum_macros"]',
            ],
        }),
    );
}

function main() {
    const project = process.argv.slice(2)[0] ?? undefined;
    const generateProto = process.argv.slice(3)[0] === 'true' ? true : false;

    if (project === undefined) {
        throw new Error('Project name is required.');
    }

    let idl = readJson(path.join(__dirname, 'idl.json'));
    idl.metadata = { origin: 'shank', address: '675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8' };

    let node = rootNodeFromAnchor(idl);

    generateProject(project, node, generateProto);
}

main();
