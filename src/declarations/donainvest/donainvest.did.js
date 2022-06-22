export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'deposit' : IDL.Func([IDL.Text, IDL.Text, IDL.Float64], [], []),
    'get_balance' : IDL.Func(
        [],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Float64))],
        [],
      ),
    'get_current_mock_user' : IDL.Func([], [IDL.Principal], []),
    'list_tokens' : IDL.Func(
        [],
        [
          IDL.Vec(
            IDL.Tuple(
              IDL.Text,
              IDL.Record({
                'name' : IDL.Text,
                'price' : IDL.Float64,
                'amount' : IDL.Float64,
              }),
            )
          ),
        ],
        ['query'],
      ),
    'seed' : IDL.Func([], [], []),
    'withdraw' : IDL.Func([IDL.Text, IDL.Text, IDL.Float64], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
