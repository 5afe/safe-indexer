defmodule RpcClient do 
    use Tesla
    plug Tesla.Middleware.BaseUrl, System.get_env("RPC_NODE_URL")
    plug Tesla.Middleware.JSON

    @topics %{
        IncomingEth: "0x3d0ce9bfc3ed7d6862dbb28b2dea94561fe714a1b4d019aa8af39730d1ad7c3d",
        ExecutionSuccess: "0x442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e",
        ExecutionFailure: "0x23428b18acfb3ea64b08dc0c1d296ea9c09702c09083ca5272e64d115b687d23",
        SafeMultisigTransaction: "0x19e764001f2cb3b0a6315f2efccf09084fbfcda01b50198d7d093a91e491f34b"
    }

    def topics , do: @topics

    def rpc_method(method, params) do
        {:ok , body} = Jason.encode(
            %RpcClient.Request{method: method, params: params});
        IO.inspect(body);
        {:ok, response} = post("", body, headers: [{"content-type", "application/json"}]);
        response.body["result"]
        # |> SafeIndexer.HexUtils.from_string
    end
end
