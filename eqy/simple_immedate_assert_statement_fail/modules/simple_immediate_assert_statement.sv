module miter (
  input  [  0:0] \__pi_dummy ,
`ifdef DIRECT_CROSS_POINTS
`else
`endif
);
  \gold.simple_immediate_assert_statement gold (
    .\__pi_dummy (\__pi_dummy ),
`ifdef DIRECT_CROSS_POINTS
`else
`endif
  );
  \gate.simple_immediate_assert_statement gate (
    .\__pi_dummy (\__pi_dummy ),
`ifdef DIRECT_CROSS_POINTS
`else
`endif
  );
`ifdef ASSUME_DEFINED_INPUTS
  miter_def_prop #(1, "assume") \__pi_dummy__assume (\__pi_dummy );
`endif
`ifndef DIRECT_CROSS_POINTS
`endif
`ifdef CHECK_MATCH_POINTS
`endif
`ifdef CHECK_OUTPUTS
`endif
`ifdef COVER_DEF_CROSS_POINTS
  `ifdef DIRECT_CROSS_POINTS
  `else
  `endif
`endif
`ifdef COVER_DEF_GOLD_MATCH_POINTS
`endif
`ifdef COVER_DEF_GATE_MATCH_POINTS
`endif
`ifdef COVER_DEF_GOLD_OUTPUTS
`endif
`ifdef COVER_DEF_GATE_OUTPUTS
`endif
endmodule
module miter_cmp_prop #(parameter WIDTH=1, parameter TYPE="assert") (input [WIDTH-1:0] in_gold, in_gate);
  reg okay;
  integer i;
  always @* begin
    okay = 1;
    for (i = 0; i < WIDTH; i = i+1)
      okay = okay && (in_gold[i] === 1'bx || in_gold[i] === in_gate[i]);
  end
  generate
    if (TYPE == "assert") always @* assert(okay);
    if (TYPE == "assume") always @* assume(okay);
    if (TYPE == "cover")  always @* cover(okay);
  endgenerate
endmodule
module miter_def_prop #(parameter WIDTH=1, parameter TYPE="assert") (input [WIDTH-1:0] in);
  wire okay = ^in !== 1'bx;
  generate
    if (TYPE == "assert") always @* assert(okay);
    if (TYPE == "assume") always @* assume(okay);
    if (TYPE == "cover")  always @* cover(okay);
  endgenerate
endmodule
module \gold.simple_immediate_assert_statement (
  input  [  0:0] \__pi_dummy
);
endmodule
module \gate.simple_immediate_assert_statement (
  input  [  0:0] \__pi_dummy
);
endmodule
