defmodule Calculadora do
    def iniciar do
        IO.puts("Calculadora Elixir")

        numero1 = lernumero("Digite o primeiro numero: ")
        numero2 = lernumero("Digite o segundo numero:")

        IO.puts("""
        Qual operação deseja fazer?
        1 - Soma
        2 - Subtracao
        3 - Multiplicacao
        4 - Divisao
        """)

        operacao = IO.gets("Escolha um numero de 1 a 4: ") |> String.trim()

        resultado =
            case operacao do
                "1" -> numero1 + numero2
                "2" -> numero1 - numero2
                "3" -> numero1 * numero2
                "4" ->
                    if numero2 != 0 do
                        numero1 / numero2
                    else
                        "Erro, divisao por zero"
                    end
                _ -> "Operacao invalida"
            end

        IO.puts("Resultado: #{resultado}")
    end

    defp lernumero(mensagem) do
        #escreve mensagem passada e transforma em string sem \n
        IO.write(mensagem)
        entrada = IO.gets("") |> String.trim()
        #o parse do numero gera uma tupla {numero, "o que sobra"}, se for um numero e mais nada retorna o numero,
        #se for outra coisa pede a entrada de novo
        case Integer.parse(entrada) do
            {numero, ""} -> numero
            _ ->
                IO.puts("Valor invalido, tente novamente")
                lernumero(mensagem)
        end
    end
end

Calculadora.iniciar()
