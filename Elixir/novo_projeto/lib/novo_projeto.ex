defmodule NovoProjeto do
  def ola(nome) do
    "Ola, #{nome}!"
  end

  def dizer_idade(idade) do
    "Voce tem #{idade} anos!"
  end
end

# Retorna "Ola, {seu nome}!" para o terminal
IO.write("Escreva seu nome:\n")
nome = IO.gets("") |> String.trim()
IO.puts(NovoProjeto.ola(nome))

# Retorna Voce tem {sua idade} anos! para o terminal
IO.write("\nDigite sua idade:\n")
idade = IO.gets("") |> String.trim() |> String.to_integer()
IO.puts(NovoProjeto.dizer_idade(idade))
