import { getAllAccounts, getAllInstructionsWithSubs, getAllPrograms, isNode, VALUE_NODES } from '@codama/nodes';
import { RenderMap } from '@codama/renderers-core';
import {
    extendVisitor,
    LinkableDictionary,
    NodeStack,
    pipe,
    recordLinkablesOnFirstVisitVisitor,
    recordNodeStackVisitor,
    staticVisitor,
} from '@codama/visitors-core';

import { getImportFromFactory, LinkOverrides, render } from './utils';
import { renderValueNode } from './renderValueNodeVisitor';
import { ImportMap } from './ImportMap';

export type GetRenderMapOptions = {
    linkOverrides?: LinkOverrides;
    renderParentInstructions?: boolean;
    codamaSdkName?: string;
};

type ParserAccountNode = {
    name: string;
};

type ParserInstructionAccountNode = {
    name: string;
    index: number;
};

type ParserInstructionNode = {
    discriminator: string | null;
    name: string;
    accounts: ParserInstructionAccountNode[];
    hasArgs: boolean;
};

export function getRenderMapVisitor(options: GetRenderMapOptions = {}) {
    const linkables = new LinkableDictionary();
    const stack = new NodeStack();

    const renderParentInstructions = options.renderParentInstructions ?? false;

    return pipe(
        staticVisitor(() => new RenderMap(), {
            keys: ['rootNode', 'programNode', 'instructionNode', 'accountNode', 'definedTypeNode'],
        }),
        v =>
            extendVisitor(v, {
                visitRoot(node) {
                    const programsToExport = getAllPrograms(node);
                    const programName = programsToExport[0]?.name;
                    const getImportFrom = getImportFromFactory(options.linkOverrides ?? {});
                    const programAccounts = getAllAccounts(node);
                    const accounts: ParserAccountNode[] = programAccounts.map(acc => {
                        return {
                            name: acc.name,
                        };
                    });
                    const progranInstructions = getAllInstructionsWithSubs(node, {
                        leavesOnly: !renderParentInstructions,
                    });
                    let IX_DATA_OFFSET = 1;

                    const instructions: ParserInstructionNode[] = progranInstructions.map(ix => {
                        let discriminator = null;
                        let discriminatorIx = ix.arguments.find(arg => arg.name === 'discriminator');
                        if (discriminatorIx) {
                            const hasDefaultValue =
                                discriminatorIx.defaultValue && isNode(discriminatorIx.defaultValue, VALUE_NODES);

                            if (hasDefaultValue) {
                                const { imports: _, render: value } = renderValueNode(
                                    discriminatorIx.defaultValue,
                                    getImportFrom,
                                );

                                discriminator = value;
                                if (Array.isArray(value)) {
                                    IX_DATA_OFFSET = value.length;
                                }
                            }
                        }

                        return {
                            discriminator,
                            name: ix.name,
                            hasArgs: ix.arguments.length > 0,
                            accounts: ix.accounts.map((acc, accIdx) => {
                                return {
                                    name: acc.name,
                                    index: accIdx,
                                };
                            }),
                        };
                    });

                    const codamaSdkName = options.codamaSdkName ?? `${programName}_program_sdk`;

                    const accountParserImports = new ImportMap();

                    accountParserImports.add('borsh::{BorshDeserialize, BorshSerialize}');

                    accounts.forEach(acc => {
                        accountParserImports.add(
                            `${codamaSdkName}::accounts::{${acc.name}::${toPascalCase(acc.name)}}`,
                        );
                    });

                    const instructionParserImports = new ImportMap();

                    instructionParserImports.add('borsh::{BorshDeserialize, BorshSerialize}');

                    instructions.forEach(ix => {
                        const ixPascalName = fromCamelCasetoPascalCase(ix.name);
                        const ixSnakeName = toSnakeCase(ix.name);
                        const ixAccounts = `${ixPascalName} as ${ixPascalName}IxAccounts`;

                        if (ix.hasArgs) {
                            const ixData = `${ixPascalName}InstructionArgs as ${ixPascalName}IxData`;
                            instructionParserImports.add(
                                `${codamaSdkName}::instructions::{${ixSnakeName}::{${ixAccounts}, ${ixData}}}`,
                            );
                        } else {
                            instructionParserImports.add(
                                `${codamaSdkName}::instructions::{${ixSnakeName}::${ixAccounts}}`,
                            );
                        }
                    });

                    const ixCtx = {
                        imports: instructionParserImports,
                        programName,
                        accounts,
                        instructions,
                        IX_DATA_OFFSET,
                    };

                    const accCtx = {
                        imports: accountParserImports,
                        programName,
                        accounts,
                    };

                    const map = new RenderMap();

                    if (accCtx.accounts.length > 0) {
                        map.add('accounts_parser.rs', render('accountsParserPage.njk', accCtx));
                    }

                    if (ixCtx.instructions.length > 0) {
                        map.add('instructions_parser.rs', render('instructionsParserPage.njk', ixCtx));
                    }

                    map.add(
                        'mod.rs',
                        render('rootMod.njk', {
                            hasAccounts: accCtx.accounts.length > 0,
                        }),
                    );

                    return map;
                },
            }),
        v => recordNodeStackVisitor(v, stack),
        v => recordLinkablesOnFirstVisitVisitor(v, linkables),
    );
}

export const fromCamelCasetoPascalCase = (str: string) => {
    return str.charAt(0).toUpperCase() + str.slice(1);
};

export const toPascalCase = (str: string) => {
    return str
        .replace(/(?:^\w|[A-Z]|\b\w)/g, (word, index) => {
            return index === 0 ? word.toUpperCase() : word.toLowerCase();
        })
        .replace(/\s+/g, '');
};

export const toSnakeCase = (str: string) => {
    return str
        .replace(/(?:^\w|[A-Z]|\b\w)/g, (word, index) => {
            return index === 0 ? word.toLowerCase() : `_${word.toLowerCase()}`;
        })
        .replace(/\s+/g, '');
};