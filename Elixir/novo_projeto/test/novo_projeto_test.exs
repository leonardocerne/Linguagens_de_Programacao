defmodule NovoProjetoTest do
  use ExUnit.Case
  doctest NovoProjeto

  test "greets the world" do
    assert NovoProjeto.hello() == :world
  end
end
