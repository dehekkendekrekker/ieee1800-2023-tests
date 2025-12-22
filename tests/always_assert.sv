//----------------------------------------------------------------
// Tests support of simple_immediate_assertions: assert()
//----------------------------------------------------------------
module simple_immediate_assert_statement(
	input wire clk
);


always @(posedge clk) begin
	assert(1);
end

endmodule
