defmodule SafeIndexer.ScriptStart do
    def main(_args) do
        response = SafeIndexer.RpcClient.sync();
        IO.inspect(response)
        # GenServer.call(SafeIndexer.Requester, {:request, "asdf", "Asdf"}, :infinity)
    end
end
