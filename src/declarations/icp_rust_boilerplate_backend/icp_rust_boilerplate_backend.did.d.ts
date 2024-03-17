import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Adopter {
  'id' : bigint,
  'name' : string,
  'contact_details' : string,
  'desired_animal_type' : string,
}
export interface Animal {
  'id' : bigint,
  'age' : number,
  'name' : string,
  'is_adopted' : boolean,
  'species' : string,
}
export type Error = { 'InvalidInput' : { 'msg' : string } } |
  { 'NotFound' : { 'msg' : string } };
export type Result = { 'Ok' : Animal } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : Adopter } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_animal' : ActorMethod<[string, string, number], Result>,
  'adopt_animal' : ActorMethod<[bigint, bigint], Result_1>,
  'delete_adopter' : ActorMethod<[bigint], Result_1>,
  'delete_animal' : ActorMethod<[bigint], Result_1>,
  'get_adopter' : ActorMethod<[bigint], Result_2>,
  'get_animal' : ActorMethod<[bigint], Result>,
  'list_adopters' : ActorMethod<[], Array<Adopter>>,
  'list_adopters_of_animal' : ActorMethod<[bigint], Array<Adopter>>,
  'list_animals' : ActorMethod<[], Array<Animal>>,
  'register_adopter' : ActorMethod<[string, string, string], Result_2>,
  'release_animal' : ActorMethod<[bigint], Result_1>,
  'update_adopter' : ActorMethod<[bigint, string, string, string], Result_2>,
  'update_animal' : ActorMethod<[bigint, string, string, number], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
