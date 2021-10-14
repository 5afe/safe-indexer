defmodule SafeIndexer.HexUtils do
    
    def from_string(input) do
        if String.starts_with?(input, "0x") do
            input 
                |> String.slice(2..-1)
                |> Integer.parse(16)
        else
            Integer.parse(input, 16)
        end
    end
end
