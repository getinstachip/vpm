module bimpy #(

		parameter BW = 18, 
		parameter LUTB = 2  

	) (

		input wire                i_clk, i_reset, i_ce,
		input wire [(LUTB-1):0]   i_a,
		input wire [(BW-1):0]     i_b,
		output reg [(BW+LUTB-1):0] o_r

	);


	// {{{
	wire [(BW+LUTB-2):0] w_r;
	wire [(BW+LUTB-3):1] c;
	// }}}

	assign w_r = { ((i_a[1]) ? i_b : {(BW){1'b0}}), 1'b0 }
				^ { 1'b0, ((i_a[0]) ? i_b : {(BW){1'b0}}) };
	assign c = { ((i_a[1]) ? i_b[(BW-2):0] : {(BW-1){1'b0}}) }
			& ((i_a[0]) ? i_b[(BW-1):1] : {(BW-1){1'b0}});


	initial o_r = 0;
	always @(posedge i_clk)
	if (i_reset)
		o_r <= 0;
	else if (i_ce)
		o_r <= w_r + { c, 2'b0 };

`ifdef FORMAL
	reg f_past_valid;

	initial f_past_valid = 1'b0;
	always @(posedge i_clk)
	f_past_valid <= 1'b1;

`define ASSERT assert

	always @(posedge i_clk)
	if ((!f_past_valid) || ($past(i_reset)))
	begin
		`ASSERT(o_r == 0);
	end else if ($past(i_ce))
	begin
		if ($past(i_a) == 0)
		begin
			`ASSERT(o_r == 0);
		end else if ($past(i_a) == 1)
			`ASSERT(o_r == $past(i_b));

		if ($past(i_b) == 0)
		begin
			`ASSERT(o_r == 0);
		end else if ($past(i_b) == 1)
			`ASSERT(o_r[(LUTB-1):0] == $past(i_a));
	end
`endif

endmodule
