package task

import java.io.File
import java.io.InputStream
import java.io.FileOutputStream
import java.io.PrintStream

const val ERROR_SYMBOL = -1
const val EOF_SYMBOL = -2
const val REAL_SYMBOL = 24
const val PLUS_SYMBOL = 15
const val MINUS_SYMBOL = 16
const val TIMES_SYMBOL = 17
const val DIVIDE_SYMBOL = 18
const val POW_SYMBOL = 27
const val LPAREN_SYMBOL = 21
const val RPAREN_SYMBOL = 22
const val NEWLINE_SYMBOL = 23
const val FOR_SYMBOL = 33
const val FFF_SYMBOL = 3
const val FOREACH_SYMBOL = 5
const val INT_SYMBOL = 10
const val VAR_SYMBOL = 12
const val EQUALS_SYMBOL = 14
const val INTEGER_DIVIDES_SYMBOL = 28
const val VARIABLE_SYMBOL = 29
const val ASSIGN_SYMBOL = 30
const val DEFINE_SYMBOL = 31
const val TERM_SYMBOL = 32
const val TO_SYMBOL = 34
const val BEGIN_SYMBOL = 35
const val END_SYMBOL = 36
const val PRINT_SYMBOL = 37

// Definiraj razred Token
data class Token(val symbol: Int, val lexeme: String, val name: String)

// Definiraj razred Lexer
class Lexer(private val inputStream: InputStream) {
    private var currentChar: Char = ' '
    private var lineNumber: Int = 1

    private fun readNextChar() {
        val nextChar = inputStream.read()
        currentChar = if (nextChar == -1) EOF_SYMBOL.toChar() else nextChar.toChar()
    }

    private fun skipWhitespace() {
        while (currentChar.isWhitespace()) {
            if (currentChar == '\n') {
                lineNumber++
            }
            readNextChar()
        }
    }

    private fun scanReal(): Token {
        val lexeme = StringBuilder()
        while (currentChar.isDigit()) {
            lexeme.append(currentChar)
            readNextChar()
        }
        if (currentChar == '.') {
            lexeme.append(currentChar)
            readNextChar()
            while (currentChar.isDigit()) {
                lexeme.append(currentChar)
                readNextChar()
            }
        }
        return Token(REAL_SYMBOL, "\"${lexeme.toString()}\"", "real")
    }

    private fun scanIdentifier(): Token {
        val lexeme = StringBuilder()
        while (currentChar.isLetterOrDigit() || currentChar == '_') {
            lexeme.append(currentChar)
            readNextChar()
        }
        return when (lexeme.toString()) {
            "for" -> Token(FOR_SYMBOL, "\"for\"", "for")
            "var" -> Token(DEFINE_SYMBOL, "\"var\"", "define")
            "print" -> Token(PRINT_SYMBOL, "\"print\"", "print")
            else -> Token(VARIABLE_SYMBOL, "\"${lexeme.toString()}\"", "variable")
        }
    }

    fun nextToken(): Token {
        skipWhitespace()

        return when (currentChar) {
            '=' -> {
                readNextChar()
                Token(ASSIGN_SYMBOL, "\"=\"", "assign")
            }
            '+' -> {
                readNextChar()
                Token(PLUS_SYMBOL, "\"+\"", "plus")
            }
            '-' -> {
                readNextChar()
                Token(MINUS_SYMBOL, "\"-\"", "minus")
            }
            '*' -> {
                readNextChar()
                Token(TIMES_SYMBOL, "\"*\"", "times")
            }
            '/' -> {
                readNextChar()
                if (currentChar == '/') {
                    readNextChar()
                    Token(INTEGER_DIVIDES_SYMBOL, "\"//\"", "integer-divides")
                } else {
                    Token(DIVIDE_SYMBOL, "\"/\"", "divides")
                }
            }
            '^' -> {
                readNextChar()
                Token(POW_SYMBOL, "\"^\"", "pow")
            }
            '(' -> {
                readNextChar()
                Token(LPAREN_SYMBOL, "\"(\"", "lparen")
            }
            ')' -> {
                readNextChar()
                Token(RPAREN_SYMBOL, "\")\"", "rparen")
            }
            ';' -> {
                readNextChar()
                Token(TERM_SYMBOL, "\";\"", "term")
            }
            ',' -> {
                readNextChar()
                Token(TO_SYMBOL, "\",\"", "to")
            }
            '{' -> {
                readNextChar()
                Token(BEGIN_SYMBOL, "\"{\"", "begin")
            }
            '}' -> {
                readNextChar()
                Token(END_SYMBOL, "\"}\"", "end")
            }
            '\n' -> {
                readNextChar()
                Token(NEWLINE_SYMBOL, "\"\\n\"", "newline")
            }
            EOF_SYMBOL.toChar() -> Token(EOF_SYMBOL, "\"EOF\"", "eof")
            else -> {
                if (currentChar.isDigit()) {
                    scanReal()
                } else if (currentChar.isLetter() || currentChar == '_') {
                    scanIdentifier()
                } else {
                    Token(ERROR_SYMBOL, "\"${currentChar}\"", "error")
                }
            }
        }
    }
}

// Definiraj glavno funkcijo main
fun main(args: Array<String>) {
    if (args.size != 2) {
        println("Usage: program <input_file> <out_file")
        return
    }

    val inputFile = File(args[0])
    if (!inputFile.exists()) {
        println("Input file not found")
        return
    }

    val inputStream = inputFile.inputStream()
    val lexer = Lexer(inputStream)

    val outputFile = File(args[1])
    val outputStream = FileOutputStream(outputFile)
    val printStream = PrintStream(outputStream)

    var token = lexer.nextToken()
    var isValid = true
    while (token.symbol != EOF_SYMBOL) {
        if (token.symbol == ERROR_SYMBOL) {
            isValid = false
            break
        }
        printStream.print("${token.name}(${token.lexeme}) ")
        token = lexer.nextToken()
    }
    printStream.println()

    inputStream.close()
    printStream.close()
}
