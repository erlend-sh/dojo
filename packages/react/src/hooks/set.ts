import { useCallback, useState } from 'react';
import { useDojoContext } from '../provider';
import { useRPCContext } from '../provider';

import {
    useContractWrite,
    Call
} from "@starknet-react/core";
import { Account, ec, Provider, stark } from "starknet";


const provider = new Provider({ sequencer: { network: 'mainnet-alpha', baseUrl: "http://127.0.0.1:5050" }, rpc: { nodeUrl: "http://127.0.0.1:5050" } })

// this could be moved to the core...

// TODO: expose calls as an array to client so users can build a queue of calls
// TODO: loading

const account = "0x06f62894bfd81d2e396ce266b2ad0f21e0668d604e5bb1077337b6d570a54aea"

const privateKey = "0x07230b49615d175307d580c33d6fda61fc7b9aec91df0f5c1a5ebe3b8cbfee02"
const publicKey = "0x078e6e3e4a50285be0f6e8d0b8a61044033e24023df6eb95979ae4073f159ae6"

export function useSystem<T>({
    key,
}: {
    key: string;
}) {
    const [calls, setCalls] = useState<Call[]>([])
    const { write } = useContractWrite({ calls })
    // const { worldAddress } = useDojoContext();

    const { rpcProvider } = useRPCContext()

    const execute = useCallback(
        async (
            call_data: bigint[],
            system: string
        ) => {

            console.log("Execute: ", call_data, system)


            const starkKeyPair = ec.getKeyPair(privateKey);

            let stark_account;

            if (rpcProvider?.provider) stark_account = new Account(provider, account, starkKeyPair)

            console.log(stark_account)

            console.log(rpcProvider?.sequencerProvider)

            const block = rpcProvider?.provider.getBlockNumber()

            console.log(block)

            const nonce = await stark_account?.getNonce()

            console.log(nonce)

            const call = await stark_account?.execute(
                {
                    contractAddress: '0x2a79e6863214cfb96bdcb42b70eb39cdb74dd7787d2b1e792b673600892eeb2',  // ETH contract address
                    entrypoint: 'execute',
                    calldata: stark.compileCalldata({
                        name: "0x287805587e3abd3111e19b56dbe7d8b8458e3ffb95a8f272466c1072b80e519"
                    })

                },
                undefined,
                {
                    nonce: nonce,
                }
            );

            // console.log(call)

            // const account = new Account(rpcProvider?.provider, account, starkKeyPair)

            // const call: Call = {
            //     entrypoint: "execute",
            //     contractAddress: worldAddress || "",
            //     calldata: [system, ...call_data]
            // }

            // setCalls([call])
            // write()
        },
        [key]
    );

    return {
        execute
    };
}
