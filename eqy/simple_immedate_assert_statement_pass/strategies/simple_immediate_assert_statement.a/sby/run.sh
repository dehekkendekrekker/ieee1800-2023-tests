STATUS=ERROR
sby -f simple_immediate_assert_statement.a.sby > /dev/null && STATUS=$(awk '{print $1}' simple_immediate_assert_statement.a/status)
echo $STATUS > status
case $STATUS in
    PASS)
        echo "Proved equivalence of partition 'simple_immediate_assert_statement.a' using strategy 'sby'"
    ;;
    FAIL)
        echo "Could not prove equivalence of partition 'simple_immediate_assert_statement.a' using strategy 'sby': partitions not equivalent"
    ;;
    UNKNOWN)
        echo "Could not prove equivalence of partition 'simple_immediate_assert_statement.a' using strategy 'sby': equivalence unknown"
    ;;
    TIMEOUT)
        echo "Could not prove equivalence of partition 'simple_immediate_assert_statement.a' using strategy 'sby': timeout"
    ;;
    *)
        cat simple_immediate_assert_statement.a/ERROR 2> /dev/null
        echo "Execution of strategy 'sby' on partition 'simple_immediate_assert_statement.a' encountered an error."
        echo "More details can be found in 'simple_immedate_assert_statement_pass/strategies/simple_immediate_assert_statement.a/sby/simple_immediate_assert_statement.a/logfile.txt'."
        exit 1
    ;;
esac
exit 0

