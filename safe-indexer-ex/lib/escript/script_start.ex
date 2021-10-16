defmodule SafeIndexer.ScriptStart do
    def main(_args) do
        response = RpcClient.rpc_method("eth_blockNumber");
        IO.inspect(response);


        params = %RpcClient.Params{
                address: "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67",
                topics: [[RpcClient.topics()[:IncomingEth]]]
        };

        response = RpcClient.rpc_method(
            "eth_getLogs",
            params);
        IO.inspect(response);

        # GenServer.call(SafeIndexer.Requester, {:request, "asdf", "Asdf"}, :infinity)
    end
end
