defmodule RpcClient.Request do
    @derive Jason.Encoder
    defstruct [  :method, jsonrpc: "2.0", params: [], id: "1"]
end 
