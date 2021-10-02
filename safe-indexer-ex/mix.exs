defmodule SafeIndexer.MixProject do
  use Mix.Project

  def project do
    [
      app: :safe_indexer,
      version: "0.1.0",
      elixir: "~> 1.12",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      escript: escript()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      mod: {SafeIndexer, []},
      extra_applications: [:logger]
    ]
  end

  def escript() do 
    [main_module: SafeIndexer.ScriptStart]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:tesla, "~> 1.4"},
      {:hackney, "~> 1.18"},
      {:jason, ">= 1.0.0"}
    ]
  end
end
