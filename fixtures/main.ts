import { execute } from './lib.ts';

async function main(): Promise<string> {
  return await execute('world');
}

export default main;
