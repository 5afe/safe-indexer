defmodule RpcClient.Request do
    @derive Jason.Encoder
    defstruct [:method, :params, jsonrpc: "2.0", id: "1"]
end 

defmodule RpcClient.Params do
    @dervive Jason.Encoder
    defstruct [:address, :topics, fromBlock: "earliest"] 
end
