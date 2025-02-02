#[cfg(test)]
mod script_tests {
    use bsv::{Hash, OpCodes, P2PKHAddress, Script, ScriptBit};
    // #[test]
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    // fn to_hex_string() {
    //   let script_bytes = hex::decode("006a2231394878696756345179427633744870515663554551797131707a5a56646f4175744c8d2453485541207768616c65206170706561726564efbc81efbc810a4d61796265205348554120736861726befbc81f09f98aff09f98af0a0a68747470733a2f2f7477657463682e6170702f742f643435653233303233383762616235643138663534666566643736633461353462356466636338316461396436623133643832303335333838313064633565630a746578742f706c61696e04746578741f7477657463685f7477746578745f313631393336303532303332352e747874017c223150755161374b36324d694b43747373534c4b79316b683536575755374d74555235035345540b7477646174615f6a736f6e046e756c6c0375726c046e756c6c07636f6d6d656e74046e756c6c076d625f75736572046e756c6c057265706c79046e756c6c047479706504706f73740974696d657374616d70046e756c6c036170700674776574636807696e766f6963652438613262346330622d636531362d346166342d393932312d386638393334343436383938017c22313550636948473232534e4c514a584d6f53556157566937575371633768436676610d424954434f494e5f45434453412231513533564e7853316e647a56444431623834316545795a4458574c6a42735167694c5848384f4c685141527132365371784d56494852554449624b4e45686d49424e6e6e362f376a70786466685731434b61387a63356c3043497471425177557a47734b4b4f2b6e4e4c33444c45424c2f7467433657666f46413d").unwrap();
    //   let script = Script::from_bytes(script_bytes);

    //   assert_eq!(script.to, "006a2231394878696756345179427633744870515663554551797131707a5a56646f4175744c8d2453485541207768616c65206170706561726564efbc81efbc810a4d61796265205348554120736861726befbc81f09f98aff09f98af0a0a68747470733a2f2f7477657463682e6170702f742f643435653233303233383762616235643138663534666566643736633461353462356466636338316461396436623133643832303335333838313064633565630a746578742f706c61696e04746578741f7477657463685f7477746578745f313631393336303532303332352e747874017c223150755161374b36324d694b43747373534c4b79316b683536575755374d74555235035345540b7477646174615f6a736f6e046e756c6c0375726c046e756c6c07636f6d6d656e74046e756c6c076d625f75736572046e756c6c057265706c79046e756c6c047479706504706f73740974696d657374616d70046e756c6c036170700674776574636807696e766f6963652438613262346330622d636531362d346166342d393932312d386638393334343436383938017c22313550636948473232534e4c514a584d6f53556157566937575371633768436676610d424954434f494e5f45434453412231513533564e7853316e647a56444431623834316545795a4458574c6a42735167694c5848384f4c685141527132365371784d56494852554449624b4e45686d49424e6e6e362f376a70786466685731434b61387a63356c3043497471425177557a47734b4b4f2b6e4e4c33444c45424c2f7467433657666f46413d");
    // }

    #[test]
    fn to_21e8_script_hex_debug() {
        let script = Script::from_hex("20d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a20221e8825479a87c7f758875ac").unwrap();

        assert_eq!(
            script.to_extended_asm_string(),
            "OP_PUSH 32 d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 OP_PUSH 2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
    }

    #[test]
    fn to_21e8_script_hex() {
        let script = Script::from_hex("20d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a20221e8825479a87c7f758875ac").unwrap();

        assert_eq!(
            script.to_asm_string(),
            "d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
    }

    #[test]
    fn to_asm_op_return_script() {
        let script = Script::from_hex("006a4cb47b227573657248616e646c65223a226c75636b787878222c226368616e6e656c223a226d61746368222c226368616e6e656c4964223a2264757374222c2277696e6e65724964223a2239383333323836362d636435372d343166332d393537632d636433376231666237643738222c2275736572496d616765223a2268747470733a2f2f636c6f75642e68616e64636173682e696f2f75736572732f70726f66696c65506963747572652f6c75636b787878227d").unwrap();

        assert_eq!(script.to_asm_string(), "0 OP_RETURN 7b227573657248616e646c65223a226c75636b787878222c226368616e6e656c223a226d61746368222c226368616e6e656c4964223a2264757374222c2277696e6e65724964223a2239383333323836362d636435372d343166332d393537632d636433376231666237643738222c2275736572496d616765223a2268747470733a2f2f636c6f75642e68616e64636173682e696f2f75736572732f70726f66696c65506963747572652f6c75636b787878227d");
    }

    #[test]
    fn twetch_op_return_script() {
        let script = Script::from_asm_string("0 OP_RETURN 31394878696756345179427633744870515663554551797131707a5a56646f417574 676d 746578742f706c61696e 74657874 7477657463685f7477746578745f313634373533323430393832322e747874 7c 3150755161374b36324d694b43747373534c4b79316b683536575755374d74555235 534554 7477646174615f6a736f6e 6e756c6c 75726c 676d 636f6d6d656e74 6e756c6c 6d625f75736572 6e756c6c 7265706c79 6e756c6c 74797065 706f7374 74696d657374616d70 6e756c6c 617070 747765746368 696e766f696365 64373935653432622d323462342d343938372d386562382d353665343733316637636237 7c 313550636948473232534e4c514a584d6f5355615756693757537163376843667661 424954434f494e5f4543445341 314a4d6f6456736e376d7248643273646447645742566d324d4d7039517338565146 4834384e5336544d7977496c6a527662615849743041546b4246514b2b5431444b2f504f562b496f59326d53616c6576526b3562644232424876654d5a736a64422f4b3333577071484d5037717a79493958626d332b773d").unwrap();
        assert_eq!(script.to_hex(), "006a2231394878696756345179427633744870515663554551797131707a5a56646f41757402676d0a746578742f706c61696e04746578741f7477657463685f7477746578745f313634373533323430393832322e747874017c223150755161374b36324d694b43747373534c4b79316b683536575755374d74555235035345540b7477646174615f6a736f6e046e756c6c0375726c02676d07636f6d6d656e74046e756c6c076d625f75736572046e756c6c057265706c79046e756c6c047479706504706f73740974696d657374616d70046e756c6c036170700674776574636807696e766f6963652464373935653432622d323462342d343938372d386562382d353665343733316637636237017c22313550636948473232534e4c514a584d6f53556157566937575371633768436676610d424954434f494e5f454344534122314a4d6f6456736e376d7248643273646447645742566d324d4d70395173385651464c584834384e5336544d7977496c6a527662615849743041546b4246514b2b5431444b2f504f562b496f59326d53616c6576526b3562644232424876654d5a736a64422f4b3333577071484d5037717a79493958626d332b773d".to_string());
        assert_eq!(script.to_asm_string(), "0 OP_RETURN 31394878696756345179427633744870515663554551797131707a5a56646f417574 676d 746578742f706c61696e 74657874 7477657463685f7477746578745f313634373533323430393832322e747874 7c 3150755161374b36324d694b43747373534c4b79316b683536575755374d74555235 534554 7477646174615f6a736f6e 6e756c6c 75726c 676d 636f6d6d656e74 6e756c6c 6d625f75736572 6e756c6c 7265706c79 6e756c6c 74797065 706f7374 74696d657374616d70 6e756c6c 617070 747765746368 696e766f696365 64373935653432622d323462342d343938372d386562382d353665343733316637636237 7c 313550636948473232534e4c514a584d6f5355615756693757537163376843667661 424954434f494e5f4543445341 314a4d6f6456736e376d7248643273646447645742566d324d4d7039517338565146 4834384e5336544d7977496c6a527662615849743041546b4246514b2b5431444b2f504f562b496f59326d53616c6576526b3562644232424876654d5a736a64422f4b3333577071484d5037717a79493958626d332b773d".to_string());

        // assert_eq!(script.to_hex(), script1.to_hex());
        //  "006a2231394878696756345179427633744870515663554551797131707a5a56646f41757402676d0a746578742f706c61696e04746578741f7477657463685f7477746578745f313634373533333130393339372e747874017c223150755161374b36324d694b43747373534c4b79316b683536575755374d74555235035345540b7477646174615f6a736f6e046e756c6c0375726c02676d07636f6d6d656e74046e756c6c076d625f75736572046e756c6c057265706c79046e756c6c047479706504706f73740974696d657374616d70046e756c6c036170700674776574636807696e766f6963652464646438633964642d653164632d343435622d396533312d353466316130333136663733017c22313550636948473232534e4c514a584d6f53556157566937575371633768436676610d424954434f494e5f454344534122314a4d6f6456736e376d7248643273646447645742566d324d4d70395173385651464c58494d525777484632796b676a56532f7a582b52467a717a76446d726a6a6935316e5378626d4d562f53356757555535673872454842465a79544d76726c64787245573766757354334d7142374f5076784b464147434f303d");
        // println!("{}", script.to_asm_string());
        // assert_eq!(script.to_asm_string(), script1.to_asm_string());
        //  "0 OP_RETURN 31394878696756345179427633744870515663554551797131707a5a56646f417574 676d 746578742f706c61696e 74657874 7477657463685f7477746578745f313634373533323430393832322e747874 7c 3150755161374b36324d694b43747373534c4b79316b683536575755374d74555235 534554 7477646174615f6a736f6e 6e756c6c 75726c 676d 636f6d6d656e74 6e756c6c 6d625f75736572 6e756c6c 7265706c79 6e756c6c 74797065 706f7374 74696d657374616d70 6e756c6c 617070 747765746368 696e766f696365 64373935653432622d323462342d343938372d386562382d353665343733316637636237 7c 313550636948473232534e4c514a584d6f5355615756693757537163376843667661 424954434f494e5f4543445341 314a4d6f6456736e376d7248643273646447645742566d324d4d7039517338565146 4834384e5336544d7977496c6a527662615849743041546b4246514b2b5431444b2f504f562b496f59326d53616c6576526b3562644232424876654d5a736a64422f4b3333577071484d5037717a79493958626d332b773d");
    }

    #[test]
    fn from_21e8_asm_string() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        assert_eq!(
            script.to_asm_string(),
            "d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
        assert_eq!(
            script.to_extended_asm_string(),
            "OP_PUSH 32 d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 OP_PUSH 2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
    }

    #[test]
    fn from_op_return_asm_string() {
        let script = Script::from_asm_string("0 OP_RETURN 7b227573657248616e646c65223a226c75636b787878222c226368616e6e656c223a226d61746368222c226368616e6e656c4964223a2264757374222c2277696e6e65724964223a2239383333323836362d636435372d343166332d393537632d636433376231666237643738222c2275736572496d616765223a2268747470733a2f2f636c6f75642e68616e64636173682e696f2f75736572732f70726f66696c65506963747572652f6c75636b787878227d").unwrap();

        assert_eq!(script.to_asm_string(), "0 OP_RETURN 7b227573657248616e646c65223a226c75636b787878222c226368616e6e656c223a226d61746368222c226368616e6e656c4964223a2264757374222c2277696e6e65724964223a2239383333323836362d636435372d343166332d393537632d636433376231666237643738222c2275736572496d616765223a2268747470733a2f2f636c6f75642e68616e64636173682e696f2f75736572732f70726f66696c65506963747572652f6c75636b787878227d");
    }

    #[test]
    fn from_custom_asm_string() {
        let script = Script::from_asm_string("OP_NOP OP_0 ff OP_0 OP_PICK OP_2 OP_ROLL OP_DROP OP_1 OP_ROLL OP_DROP OP_NOP OP_1 OP_PICK OP_0 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_NUMEQUAL OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_1 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_LESSTHAN OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_2 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_GREATERTHAN OP_NIP OP_NIP OP_NIP OP_ELSE OP_0 OP_ENDIF OP_ENDIF OP_ENDIF").unwrap();

        assert_eq!(script.to_extended_asm_string(), "OP_NOP OP_0 OP_PUSH 1 ff OP_0 OP_PICK OP_2 OP_ROLL OP_DROP OP_1 OP_ROLL OP_DROP OP_NOP OP_1 OP_PICK OP_0 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_NUMEQUAL OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_1 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_LESSTHAN OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_2 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_GREATERTHAN OP_NIP OP_NIP OP_NIP OP_ELSE OP_0 OP_ENDIF OP_ENDIF OP_ENDIF");
        assert_eq!(script.to_asm_string(), "OP_NOP 0 ff 0 OP_PICK OP_2 OP_ROLL OP_DROP OP_1 OP_ROLL OP_DROP OP_NOP OP_1 OP_PICK 0 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_NUMEQUAL OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_1 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_LESSTHAN OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_2 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_GREATERTHAN OP_NIP OP_NIP OP_NIP OP_ELSE 0 OP_ENDIF OP_ENDIF OP_ENDIF");
    }

    #[test]
    fn from_p2pkh_asm_string() {
        let script = Script::from_asm_string("OP_DUP OP_HASH160 6fa5502ea094d59576898b490d866b32a61b89f6 OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        assert_eq!(script.to_asm_string(), "OP_DUP OP_HASH160 6fa5502ea094d59576898b490d866b32a61b89f6 OP_EQUALVERIFY OP_CHECKSIG");
    }

    #[test]
    fn from_problematic_asm_string() {
        let script = Script::from_asm_string("OP_RETURN 026d02 0568656c6c6f").unwrap();

        assert_eq!(script.to_asm_string(), "OP_RETURN 026d02 0568656c6c6f");
    }

    #[test]
    fn throw_error_with_invalid_hex() {
        let script = Script::from_asm_string("OP_RETURN 026d02 0568656c6c6fzz");

        assert!(script.is_err());
    }

    #[test]
    fn from_standard_21e8_asm_format() {
        let script =
            Script::from_asm_string("0a40eda5ff94de646c3928e4a8eff097feeb283d124b0e871b24962e75846144 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        assert_eq!(
            script.to_asm_string(),
            "0a40eda5ff94de646c3928e4a8eff097feeb283d124b0e871b24962e75846144 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
    }

    #[test]
    fn another_hex_21e8_script() {
        let script = Script::from_hex("200a40eda5ff94de646c3928e4a8eff097feeb283d124b0e871b24962e758461440221e8825479a87c7f758875ac").unwrap();

        assert_eq!(
            script.to_asm_string(),
            "0a40eda5ff94de646c3928e4a8eff097feeb283d124b0e871b24962e75846144 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
    }

    #[test]
    fn encode_pushdata_test() {
        let test_data: Vec<u8> = vec![0; 11];

        let pushdata = Script::encode_pushdata(&test_data).unwrap();
        assert_eq!(pushdata.len(), 12);
    }

    #[test]
    fn encode_pushdata_2_test() {
        let test_data: Vec<u8> = vec![0; 1024];

        let pushdata = Script::encode_pushdata(&test_data).unwrap();
        assert_eq!(pushdata.len(), 1027);
    }

    #[test]
    // Cant run in wasm, data size too beeg
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn encode_pushdata_4_test() {
        let test_data: Vec<u8> = vec![0; 3_000_000_000];

        let pushdata = Script::encode_pushdata(&test_data).unwrap();
        assert_eq!(pushdata.len(), 3000000005);
    }

    #[test]
    fn from_op_return_asm_string_long_buffer() {
        let script = Script::from_asm_string("0 OP_RETURN 7b227469746c65223a22547572626f20466f7820233633222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36332c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a22426c61636b227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d").unwrap();

        assert_eq!(script.to_hex(), "006a4d00017b227469746c65223a22547572626f20466f7820233633222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36332c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a22426c61636b227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d");

        assert_eq!(script.to_asm_string(), "0 OP_RETURN 7b227469746c65223a22547572626f20466f7820233633222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36332c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a22426c61636b227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d");

        assert_eq!(Script::from_hex(&script.to_hex()).unwrap().to_asm_string(), "0 OP_RETURN 7b227469746c65223a22547572626f20466f7820233633222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36332c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a22426c61636b227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d");
    }

    #[test]
    fn long_zero_buffer() {
        let script = Script::from_asm_string("0000000000000000000000000000000000000000000000000000000000000000").unwrap();

        assert_eq!(script.to_asm_string(), "0000000000000000000000000000000000000000000000000000000000000000");
    }

    #[test]
    fn read_p2pkh_address_to_script() {
        let address = P2PKHAddress::from_string("1PnDKikz2eGJY3F36ALD2zvpgh8G2HKpAR").unwrap();

        let p2pkh_locking_script = address.get_locking_script().unwrap();

        assert_eq!(
            p2pkh_locking_script.to_asm_string(),
            "OP_DUP OP_HASH160 f9dfc5a4ae5256e5938c2d819738f7b57e4d7b46 OP_EQUALVERIFY OP_CHECKSIG"
        );

        assert_eq!(p2pkh_locking_script.to_hex(), "76a914f9dfc5a4ae5256e5938c2d819738f7b57e4d7b4688ac");

        assert_eq!(
            Script::from_hex(&p2pkh_locking_script.to_hex()).unwrap().to_asm_string(),
            "OP_DUP OP_HASH160 f9dfc5a4ae5256e5938c2d819738f7b57e4d7b46 OP_EQUALVERIFY OP_CHECKSIG"
        );
    }

    #[test]
    fn from_asm_string_custom_script() {
        let address = P2PKHAddress::from_string("1PnDKikz2eGJY3F36ALD2zvpgh8G2HKpAR").unwrap();
        let hash = Hash::hash_160(&[0; 32]);
        let meta = "7b227469746c65223a22547572626f20466f7820233633222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36332c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a22426c61636b227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d";

        let p2pkh_locking_hex = address.get_locking_script().unwrap().to_asm_string();
        let hash_hex = hash.to_hex();

        let script = Script::from_asm_string(&format!("OP_HASH160 {} OP_EQUALVERIFY {} OP_RETURN {}", hash_hex, p2pkh_locking_hex, meta)).unwrap();

        assert_eq!(script.to_asm_string(), format!("OP_HASH160 {} OP_EQUALVERIFY {} OP_RETURN {}", hash_hex, p2pkh_locking_hex, meta));
    }

    #[test]
    fn from_hex_with_large_buffer_script() {
        let actual =  Script::from_hex("a914b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc68876a914f9dfc5a4ae5256e5938c2d819738f7b57e4d7b4688ac6a4d01017b227469746c65223a22547572626f20466f7820233631222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36312c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a224f72616e6765227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d").unwrap();

        assert_eq!(
            actual.to_asm_string(),
            "OP_HASH160 b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6 OP_EQUALVERIFY OP_DUP OP_HASH160 f9dfc5a4ae5256e5938c2d819738f7b57e4d7b46 OP_EQUALVERIFY OP_CHECKSIG OP_RETURN 7b227469746c65223a22547572626f20466f7820233631222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36312c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a224f72616e6765227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d",
            "Expected ASM string didnt match hardcoded ASM string"
        );
    }

    #[test]
    fn from_asm_string_with_large_buffer_script() {
        let actual =  Script::from_asm_string("OP_HASH160 b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6 OP_EQUALVERIFY OP_DUP OP_HASH160 f9dfc5a4ae5256e5938c2d819738f7b57e4d7b46 OP_EQUALVERIFY OP_CHECKSIG OP_RETURN 7b227469746c65223a22547572626f20466f7820233631222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36312c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a224f72616e6765227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d").unwrap();

        assert_eq!(
            actual.to_asm_string(),
            "OP_HASH160 b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6 OP_EQUALVERIFY OP_DUP OP_HASH160 f9dfc5a4ae5256e5938c2d819738f7b57e4d7b46 OP_EQUALVERIFY OP_CHECKSIG OP_RETURN 7b227469746c65223a22547572626f20466f7820233631222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36312c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a224f72616e6765227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d",
            "Expected ASM string didnt match hardcoded ASM string"
        );
    }

    #[test]
    fn from_hex_custom_script() {
        let actual =  Script::from_hex("a914b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc68876a91414a8036c8b3d910a7e24d46067048d8761274b5588ac6a4d00017b227469746c65223a22547572626f20466f7820233633222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36332c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a22426c61636b227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d").unwrap();

        assert_eq!(
            actual.to_asm_string(),
            "OP_HASH160 b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6 OP_EQUALVERIFY OP_DUP OP_HASH160 14a8036c8b3d910a7e24d46067048d8761274b55 OP_EQUALVERIFY OP_CHECKSIG OP_RETURN 7b227469746c65223a22547572626f20466f7820233633222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a36332c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a22426c61636b227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d"
        );
    }

    #[test]
    fn if_statement_script() {
        let script = Script::from_asm_string(
            r#"21e8
            OP_IF 
                OP_1 OP_RETURN 
            OP_ELSE 
                OP_0 OP_RETURN 
            OP_ENDIF"#,
        )
        .unwrap();

        assert_eq!(
            &script.to_script_bits(),
            &[
                ScriptBit::Push(hex::decode("21e8").unwrap()),
                ScriptBit::If {
                    code: OpCodes::OP_IF,
                    pass: vec![ScriptBit::OpCode(OpCodes::OP_1), ScriptBit::OpCode(OpCodes::OP_RETURN)],
                    fail: Some(vec![ScriptBit::OpCode(OpCodes::OP_0), ScriptBit::OpCode(OpCodes::OP_RETURN)]),
                }
            ]
        );
    }

    //     #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    //     #[cfg(target_arch = "wasm32")]
    //     fn if_statement_script() {
    //         let script = Script::from_asm_string(
    //             r#"21e8
    //             OP_IF
    //                 OP_1 OP_RETURN
    //             OP_ELSE
    //                 OP_0 OP_RETURN
    //             OP_ENDIF"#,
    //         )
    //         .unwrap();

    //         assert_eq!(
    //             format!("{:?}", &script.to_script_bits().unwrap()),
    //             format!(
    //                 "{:?}",
    //                 &serde_wasm_bindgen::to_value(&[
    //                     ScriptBit::Push(hex::decode("21e8").unwrap()),
    //                     ScriptBit::If {
    //                         code: OpCodes::OP_IF,
    //                         pass: vec![ScriptBit::OpCode(OpCodes::OP_1), ScriptBit::OpCode(OpCodes::OP_RETURN)],
    //                         fail: vec![ScriptBit::OpCode(OpCodes::OP_0), ScriptBit::OpCode(OpCodes::OP_RETURN)],
    //                     }
    //                 ])
    //                 .unwrap()
    //             )
    //         );
    //     }

    #[test]
    fn double_nested_if_statement_script() {
        let script = Script::from_asm_string(
            r#"
            21e8 
            OP_IF 
                OP_1
                OP_IF 
                    OP_2 OP_RETURN 
                OP_ELSE 
                    OP_0 OP_RETURN 
                OP_ENDIF
            OP_ELSE 
                OP_1
                OP_IF 
                    OP_3 OP_RETURN 
                OP_ELSE 
                    OP_0 OP_RETURN 
                OP_ENDIF
            OP_ENDIF"#,
        )
        .unwrap();

        assert_eq!(
            &script.to_script_bits(),
            &[
                ScriptBit::Push(hex::decode("21e8").unwrap()),
                ScriptBit::If {
                    code: OpCodes::OP_IF,
                    pass: vec![
                        ScriptBit::OpCode(OpCodes::OP_1),
                        ScriptBit::If {
                            code: OpCodes::OP_IF,
                            pass: vec![ScriptBit::OpCode(OpCodes::OP_2), ScriptBit::OpCode(OpCodes::OP_RETURN)],
                            fail: Some(vec![ScriptBit::OpCode(OpCodes::OP_0), ScriptBit::OpCode(OpCodes::OP_RETURN)]),
                        }
                    ],
                    fail: Some(vec![
                        ScriptBit::OpCode(OpCodes::OP_1),
                        ScriptBit::If {
                            code: OpCodes::OP_IF,
                            pass: vec![ScriptBit::OpCode(OpCodes::OP_3), ScriptBit::OpCode(OpCodes::OP_RETURN)],
                            fail: Some(vec![ScriptBit::OpCode(OpCodes::OP_0), ScriptBit::OpCode(OpCodes::OP_RETURN)]),
                        }
                    ]),
                }
            ]
        );
    }

    #[test]
    fn double_nested_verif_statement_script() {
        let script = Script::from_asm_string(
            r#"
            21e8 
            OP_IF 
                OP_1
                OP_VERIF 
                    OP_2 OP_RETURN 
                OP_ELSE 
                    OP_0 OP_RETURN 
                OP_ENDIF
            OP_ELSE 
                OP_1
                OP_VERIF 
                    OP_3 OP_RETURN 
                OP_ELSE 
                    OP_0 OP_RETURN 
                OP_ENDIF
            OP_ENDIF"#,
        )
        .unwrap();

        assert_eq!(
            &script.to_script_bits(),
            &[
                ScriptBit::Push(hex::decode("21e8").unwrap()),
                ScriptBit::If {
                    code: OpCodes::OP_IF,
                    pass: vec![
                        ScriptBit::OpCode(OpCodes::OP_1),
                        ScriptBit::If {
                            code: OpCodes::OP_VERIF,
                            pass: vec![ScriptBit::OpCode(OpCodes::OP_2), ScriptBit::OpCode(OpCodes::OP_RETURN)],
                            fail: Some(vec![ScriptBit::OpCode(OpCodes::OP_0), ScriptBit::OpCode(OpCodes::OP_RETURN)]),
                        }
                    ],
                    fail: Some(vec![
                        ScriptBit::OpCode(OpCodes::OP_1),
                        ScriptBit::If {
                            code: OpCodes::OP_VERIF,
                            pass: vec![ScriptBit::OpCode(OpCodes::OP_3), ScriptBit::OpCode(OpCodes::OP_RETURN)],
                            fail: Some(vec![ScriptBit::OpCode(OpCodes::OP_0), ScriptBit::OpCode(OpCodes::OP_RETURN)]),
                        }
                    ]),
                }
            ]
        );
    }

    #[test]
    fn unterminated_else_if_statement_fails() {
        let script = Script::from_asm_string(
            r#"21e8
            OP_IF"#,
        );

        assert!(script.is_err())
    }

    #[test]
    fn unterminated_endif_if_statement_fails() {
        let script = Script::from_asm_string(
            r#"21e8
            OP_IF 
                OP_1 OP_RETURN 
            OP_ELSE 
                OP_0 OP_RETURN"#,
        );

        assert!(script.is_err())
    }

    #[test]
    fn op_if_without_else_statement_script() {
        let script = Script::from_asm_string(
            r#"
            OP_0 
            OP_IF 
                OP_2 OP_RETURN
            OP_ENDIF
            OP_1 OP_RETURN
            "#,
        )
        .unwrap();

        assert_eq!(
            &script.to_script_bits(),
            &[
                ScriptBit::OpCode(OpCodes::OP_0),
                ScriptBit::If {
                    code: OpCodes::OP_IF,
                    pass: vec![ScriptBit::OpCode(OpCodes::OP_2), ScriptBit::OpCode(OpCodes::OP_RETURN)],
                    fail: None,
                },
                ScriptBit::OpCode(OpCodes::OP_1),
                ScriptBit::OpCode(OpCodes::OP_RETURN)
            ]
        );

        assert_eq!(&script.to_asm_string(), "0 OP_IF OP_2 OP_RETURN OP_ENDIF OP_1 OP_RETURN")
    }

    #[test]
    fn empty_op_if_branches() {
        let script = Script::from_asm_string(
            r#"
            OP_1
            OP_IF 
            OP_ELSE
            OP_ENDIF
            "#,
        )
        .unwrap();

        assert_eq!(
            &script.to_script_bits(),
            &[
                ScriptBit::OpCode(OpCodes::OP_1),
                ScriptBit::If {
                    code: OpCodes::OP_IF,
                    pass: vec![],
                    fail: Some(vec![]),
                },
            ]
        );

        assert_eq!(&script.to_asm_string(), "OP_1 OP_IF OP_ELSE OP_ENDIF")
    }

    #[test]
    fn empty_op_endif_branches() {
        let script = Script::from_asm_string(
            r#"
            OP_1
            OP_IF
            OP_ENDIF
            "#,
        )
        .unwrap();

        assert_eq!(
            &script.to_script_bits(),
            &[
                ScriptBit::OpCode(OpCodes::OP_1),
                ScriptBit::If {
                    code: OpCodes::OP_IF,
                    pass: vec![],
                    fail: None,
                },
            ]
        );

        assert_eq!(&script.to_asm_string(), "OP_1 OP_IF OP_ENDIF")
    }

    #[test]
    fn scrypt_stateful_contract() {
        let script = Script::from_hex("6a00010100010001000100010001000100010001001400000000").unwrap();

        assert_eq!(
            &script.to_script_bits(),
            &[
                ScriptBit::OpCode(OpCodes::OP_RETURN),
                ScriptBit::OpCode(OpCodes::OP_0),
                ScriptBit::Push(hex::decode("01").unwrap()),
                ScriptBit::OpCode(OpCodes::OP_0),
                ScriptBit::Push(hex::decode("00").unwrap()),
                ScriptBit::Push(hex::decode("00").unwrap()),
                ScriptBit::Push(hex::decode("00").unwrap()),
                ScriptBit::Push(hex::decode("00").unwrap()),
                ScriptBit::Push(hex::decode("00").unwrap()),
                ScriptBit::Push(hex::decode("00").unwrap()),
                ScriptBit::Push(hex::decode("00").unwrap()),
                ScriptBit::Push(hex::decode("00").unwrap()),
                ScriptBit::Push(hex::decode("00000000").unwrap()),
            ]
        );

        assert_eq!(&script.to_asm_string(), "OP_RETURN 0 01 0 00 00 00 00 00 00 00 00 00000000")
    }

    #[test]
    fn byte_serialization_with_if() {
        let asm = "OP_1 OP_IF OP_1 OP_ELSE OP_0 OP_ENDIF OP_DROP";
        let script = Script::from_asm_string(&asm).unwrap();
        let script_bytes = script.to_bytes();
        let script2 = Script::from_bytes(&script_bytes).unwrap();
        let script2_asm = script2.to_extended_asm_string();
        assert_eq!(asm, script2_asm);
    }
}
