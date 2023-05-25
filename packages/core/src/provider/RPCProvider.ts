import { RpcProvider, SequencerProvider } from "starknet";
import { Call } from "starknet";
import { Provider } from "./provider";
import { Query, WorldEntryPoints } from "../types";


export class RPCProvider extends Provider {
    public provider: RpcProvider;
    public sequencerProvider: SequencerProvider;
    private loggingEnabled: boolean;

    constructor(world_address: string, url: string, loggingEnabled = false) {
        super(world_address);
        this.provider = new RpcProvider({
            nodeUrl: url,
        });
        this.sequencerProvider = new SequencerProvider({
            baseUrl: url,
            feederGatewayUrl: 'feeder_gateway',
            gatewayUrl: 'gateway',
        })
        this.loggingEnabled = loggingEnabled;
    }

    // private log(level: string, message: string) {
    //     if (this.loggingEnabled) {
    //         logger.log(level, message);
    //     }
    // }

    // fetches a component of an entity
    public async entity(component: string, query: Query, offset: number, length: number): Promise<Array<bigint>> {

        const call_data = [component, query.partition, ...query.keys, offset, length]

        const call: Call = {
            entrypoint: WorldEntryPoints.get,
            contractAddress: this.getWorldAddress(),
            calldata: call_data
        }

        try {
            const response = await this.provider.callContract(call);

            return response.result as unknown as Array<bigint>;
        } catch (error) {
            // this.log("error", `Entity call failed: ${error}`);

            throw error;
        }
    }
}