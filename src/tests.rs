#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::ContractError;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{attr, from_binary, Empty, Env, MessageInfo, OwnedDeps, Response};

    pub const ADDR1: &str = "juno1gjqnuhv52pd2a7ets2vhw9w9qa9knyhyqd4qeg";;
    pub const ADDR2: &str = "juno1chgwz55h9kepjq0fkj5supl2ta3nwu638camkg";

    pub const VALUE1: u8 = 42;
    pub const VALUE2: u8 = 45;

    type Instance = (
        OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        Env,
        MessageInfo,
        Result<Response, ContractError>,
    );

    fn get_instance(count: u8, addr: &str) -> Instance {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(addr, &[]);
        let msg = InstantiateMsg { count };

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);
        (deps, env, info, res)
    }

    #[test]
    fn test_init() {
        let (_, _, _, res) = get_instance(VALUE1, ADDR1);

        assert_eq!(
            res.unwrap().attributes,
            vec![
                attr("method", "instantiate"),
                attr("owner", ADDR1.to_string()),
                attr("count", VALUE1.to_string())
            ]
        )
    }

    #[test]
    fn test_set() {
        let (mut deps, env, info, _) = get_instance(VALUE1, ADDR1);
        let msg = ExecuteMsg::Set { count: VALUE2 };
        let res = execute(deps.as_mut(), env, info, msg);

        assert_eq!(
            res.unwrap().attributes,
            vec![
                attr("method", "set"),
                attr("owner", ADDR1.to_string()),
                attr("count", VALUE2.to_string())
            ]
        )
    }

    #[test]
    fn test_query() {
        let (deps, env, _, _) = get_instance(VALUE1, ADDR1);
        let msg = QueryMsg::GetCount {};
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res = from_binary::<CountResponse>(&bin).unwrap();

        assert_eq!(res.count, VALUE1);
    }
}
