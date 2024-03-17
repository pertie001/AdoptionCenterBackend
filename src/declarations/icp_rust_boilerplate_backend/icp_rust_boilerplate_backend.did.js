export const idlFactory = ({ IDL }) => {
  const Animal = IDL.Record({
    'id' : IDL.Nat64,
    'age' : IDL.Nat32,
    'name' : IDL.Text,
    'is_adopted' : IDL.Bool,
    'species' : IDL.Text,
  });
  const Error = IDL.Variant({
    'InvalidInput' : IDL.Record({ 'msg' : IDL.Text }),
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : Animal, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const Adopter = IDL.Record({
    'id' : IDL.Nat64,
    'name' : IDL.Text,
    'contact_details' : IDL.Text,
    'desired_animal_type' : IDL.Text,
  });
  const Result_2 = IDL.Variant({ 'Ok' : Adopter, 'Err' : Error });
  return IDL.Service({
    'add_animal' : IDL.Func([IDL.Text, IDL.Text, IDL.Nat32], [Result], []),
    'adopt_animal' : IDL.Func([IDL.Nat64, IDL.Nat64], [Result_1], []),
    'delete_adopter' : IDL.Func([IDL.Nat64], [Result_1], []),
    'delete_animal' : IDL.Func([IDL.Nat64], [Result_1], []),
    'get_adopter' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_animal' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'list_adopters' : IDL.Func([], [IDL.Vec(Adopter)], ['query']),
    'list_adopters_of_animal' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(Adopter)],
        ['query'],
      ),
    'list_animals' : IDL.Func([], [IDL.Vec(Animal)], ['query']),
    'register_adopter' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text],
        [Result_2],
        [],
      ),
    'release_animal' : IDL.Func([IDL.Nat64], [Result_1], []),
    'update_adopter' : IDL.Func(
        [IDL.Nat64, IDL.Text, IDL.Text, IDL.Text],
        [Result_2],
        [],
      ),
    'update_animal' : IDL.Func(
        [IDL.Nat64, IDL.Text, IDL.Text, IDL.Nat32],
        [Result],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
