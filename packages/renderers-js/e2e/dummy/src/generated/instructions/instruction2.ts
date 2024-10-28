/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  AccountRole,
  type Address,
  type IAccountMeta,
  type IInstruction,
  type IInstructionWithAccounts,
} from '@solana/web3.js';
import { DUMMY_PROGRAM_ADDRESS } from '../programs';

export type Instruction2Instruction<
  TProgram extends string = typeof DUMMY_PROGRAM_ADDRESS,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> & IInstructionWithAccounts<TRemainingAccounts>;

export type Instruction2Input = {
  remainingAccounts?: Array<Address>;
};

export function getInstruction2Instruction<
  TProgramAddress extends Address = typeof DUMMY_PROGRAM_ADDRESS,
>(
  input: Instruction2Input,
  config?: { programAddress?: TProgramAddress }
): Instruction2Instruction<TProgramAddress> {
  // Program address.
  const programAddress = config?.programAddress ?? DUMMY_PROGRAM_ADDRESS;

  // Original args.
  const args = { ...input };

  // Remaining accounts.
  const remainingAccounts: IAccountMeta[] = (args.remainingAccounts ?? []).map(
    (address) => ({ address, role: AccountRole.READONLY })
  );

  const instruction = {
    accounts: remainingAccounts,
    programAddress,
  } as Instruction2Instruction<TProgramAddress>;

  return instruction;
}

export type ParsedInstruction2Instruction<
  TProgram extends string = typeof DUMMY_PROGRAM_ADDRESS,
> = {
  programAddress: Address<TProgram>;
};

export function parseInstruction2Instruction<TProgram extends string>(
  instruction: IInstruction<TProgram>
): ParsedInstruction2Instruction<TProgram> {
  return {
    programAddress: instruction.programAddress,
  };
}