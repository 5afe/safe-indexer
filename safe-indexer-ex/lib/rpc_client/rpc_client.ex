defmodule SafeIndexer.RpcClient do 
    use Tesla
    plug Tesla.Middleware.BaseUrl, System.get_env("RPC_NODE_URL")
    plug Tesla.Middleware.JSON

    def sync() do
        {:ok , body} = Jason.encode(%{"jsonrpc" => "2.0", "method" => "eth_syncing", "params" => [], "id" => 1}); 
        post("", body, headers: [{"content-type", "application/json"}])
    end
end
