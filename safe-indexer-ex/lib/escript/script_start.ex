defmodule SafeIndexer.ScriptStart do
    def main(_args) do
        GenServer.call(SafeIndexer.Requester, {:request, "asdf", "Asdf"}, :infinity)
    end
end
