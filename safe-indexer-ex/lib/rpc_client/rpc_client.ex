defmodule RpcClient do 
    use Tesla
    plug Tesla.Middleware.BaseUrl, System.get_env("RPC_NODE_URL")
    plug Tesla.Middleware.JSON

    def sync() do 
        post("", '{"jsonrpc":"2.0","method":"eth_syncing","params":[],"id":1}', headers: [{"content-type", "application/json"}])
    end
end
