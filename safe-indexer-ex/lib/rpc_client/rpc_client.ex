defmodule RpcClient do 
    use Tesla
    plug Tesla.Middleware.BaseUrl, System.get_env("RPC_NODE_URL")
    plug Tesla.Middleware.JSON

    def latest_block_number() do
        {:ok , body} = Jason.encode(%RpcClient.Request{method: "eth_blockNumber"}); 
        {:ok, response} = post("", body, headers: [{"content-type", "application/json"}]);
        response.body["result"]
        |> String.slice(2..-1)
        |> Integer.parse(16)
    end
end
