.DEFAULT_GOAL := all

strategies/simple_immediate_assert_statement.a/sby/status:
	@echo "Running strategy 'sby' on 'simple_immediate_assert_statement.a'.."
	@bash -c "cd strategies/simple_immediate_assert_statement.a/sby; source run.sh"

.PHONY: all summary
all: strategies/simple_immediate_assert_statement.a/sby/status
	$(MAKE) -f strategies.mk summary
summary:
	@rc=0 ; \
	while read f; do \
		p=$${f#strategies/} ; p=$${p%/*/status} ; \
		if grep -q "PASS" $$f ; then \
			echo "* Successfully proved equivalence of partition $$p" ; \
		else \
			echo "* Failed to prove equivalence of partition $$p" ; rc=1 ; \
		fi ; \
	done < summary_targets.list ; \
	if [ "$$rc" -eq 0 ] ; then \
		echo "* Successfully proved designs equivalent" ; \
	fi
