import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'deposit' : ActorMethod<[string, string, number], undefined>,
  'get_balance' : ActorMethod<[], Array<[string, number]>>,
  'get_current_mock_user' : ActorMethod<[], Principal>,
  'list_tokens' : ActorMethod<
    [],
    Array<[string, { 'name' : string, 'price' : number, 'amount' : number }]>,
  >,
  'seed' : ActorMethod<[], undefined>,
  'withdraw' : ActorMethod<[string, string, number], undefined>,
}
