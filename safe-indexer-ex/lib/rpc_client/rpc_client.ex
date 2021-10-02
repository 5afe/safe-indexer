defmodule RpcClient do 
    use Tesla
    plug Tesla.Middleware.BaseUrl, System.get_env("RPC_NODE_URL")
    plug Tesla.Middleware.JSON

    def sync() do
        {:ok , body} = Jason.encode(%RpcClient.Request{method: "eth_syncing"}); 
        post("", body, headers: [{"content-type", "application/json"}])
    end
end
