defmodule SafeIndexer.ScriptStart do
    def main(_args) do
        response = RpcClient.latest_block_number();
        IO.inspect(response)
        # GenServer.call(SafeIndexer.Requester, {:request, "asdf", "Asdf"}, :infinity)
    end
end
