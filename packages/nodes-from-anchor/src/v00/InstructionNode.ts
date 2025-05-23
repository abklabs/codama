import {
    bytesTypeNode,
    bytesValueNode,
    camelCase,
    DiscriminatorNode,
    fieldDiscriminatorNode,
    fixedSizeTypeNode,
    instructionArgumentNode,
    InstructionNode,
    instructionNode,
    numberValueNode,
} from '@codama/nodes';

import { getAnchorInstructionDiscriminatorV00 } from '../discriminators';
import { IdlV00Instruction } from './idl';
import { instructionAccountNodesFromAnchorV00 } from './InstructionAccountNode';
import { instructionArgumentNodeFromAnchorV00 } from './InstructionArgumentNode';
import { typeNodeFromAnchorV00 } from './typeNodes';

export function instructionNodeFromAnchorV00(
    idl: IdlV00Instruction,
    ixIndex: number,
    origin?: 'anchor' | 'shank',
): InstructionNode {
    const idlName = idl.name ?? '';
    const name = camelCase(idlName);
    let dataArguments = (idl.args ?? []).map(instructionArgumentNodeFromAnchorV00);

    // Instruction discriminator.
    let discriminators: DiscriminatorNode[] | undefined;
    if (idl.discriminant) {
        const discriminatorField = instructionArgumentNode({
            defaultValue: numberValueNode(idl.discriminant.value),
            defaultValueStrategy: 'omitted',
            name: 'discriminator',
            type: typeNodeFromAnchorV00(idl.discriminant.type),
        });
        dataArguments = [discriminatorField, ...dataArguments];
        discriminators = [fieldDiscriminatorNode('discriminator')];
    } else if (origin === 'anchor') {
        const discriminatorField = instructionArgumentNode({
            defaultValue: getAnchorInstructionDiscriminatorV00(idlName),
            defaultValueStrategy: 'omitted',
            name: 'discriminator',
            type: fixedSizeTypeNode(bytesTypeNode(), 8),
        });
        dataArguments = [discriminatorField, ...dataArguments];
        discriminators = [fieldDiscriminatorNode('discriminator')];
    } else if (origin === 'shank') {
        const discriminatorField = instructionArgumentNode({
            defaultValue: bytesValueNode('base16', ixIndex.toString(16)),
            defaultValueStrategy: 'omitted',
            name: 'discriminator',
            type: fixedSizeTypeNode(bytesTypeNode(), 1),
        });
        dataArguments = [discriminatorField, ...dataArguments];
        discriminators = [fieldDiscriminatorNode('discriminator')];
    }

    return instructionNode({
        accounts: instructionAccountNodesFromAnchorV00(idl.accounts ?? []),
        arguments: dataArguments,
        discriminators,
        docs: idl.docs ?? [],
        name,
        optionalAccountStrategy: idl.legacyOptionalAccountsStrategy ? 'omitted' : 'programId',
    });
}
