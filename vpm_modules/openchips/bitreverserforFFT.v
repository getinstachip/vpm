// module NOT VERIFIED, SYNTHESIZED, OR SIMULATED

module bitreverse #(
	parameter LGSIZE = 5, 
	parameter WIDTH = 24
) (
	input  wire               i_clk, i_reset, i_ce,
	input  wire [(2*WIDTH-1):0] i_in,
	output reg  [(2*WIDTH-1):0] o_out,
	output reg                o_sync
);


	reg  [(LGSIZE):0] wraddr;
	wire [(LGSIZE):0] rdaddr;
	reg  [(2*WIDTH-1):0] brmem [0:((1<<(LGSIZE+1))-1)];
	reg in_reset;

	// Bit-reverse read address
	genvar k;
	generate 
		for (k = 0; k < LGSIZE; k = k + 1) begin : BITREV
			assign rdaddr[k] = wraddr[LGSIZE-1-k];
		end
	endgenerate
	assign rdaddr[LGSIZE] = !wraddr[LGSIZE];


	always @(posedge i_clk or posedge i_reset) begin
		if (i_reset)
			in_reset <= 1'b1;
		else if (i_ce && &wraddr[LGSIZE-1:0])
			in_reset <= 1'b0;
	end


	always @(posedge i_clk or posedge i_reset) begin
		if (i_reset)
			wraddr <= 0;
		else if (i_ce) begin
			brmem[wraddr] <= i_in;
			wraddr <= wraddr + 1;
		end
	end


	always @(posedge i_clk) begin
		if (i_ce)
			o_out <= brmem[rdaddr];
	end


	always @(posedge i_clk or posedge i_reset) begin
		if (i_reset)
			o_sync <= 1'b0;
		else if (i_ce && !in_reset)
			o_sync <= (wraddr[LGSIZE-1:0] == 0);
	end

`ifdef FORMAL
`define ASSERT assert
`define ASSUME assume

	reg f_past_valid;
	initial f_past_valid = 1'b0;
	always @(posedge i_clk)
		f_past_valid <= 1'b1;

	initial `ASSUME(i_reset);

	always @(posedge i_clk)
	if ((!f_past_valid) || ($past(i_reset))) begin
		`ASSERT(wraddr == 0);
		`ASSERT(in_reset);
		`ASSERT(!o_sync);
	end

	(* anyconst *) reg [LGSIZE:0] f_const_addr;
	wire [LGSIZE:0] f_reversed_addr;
	reg f_addr_loaded;
	reg [(2*WIDTH-1):0] f_addr_value;


	generate 
		for (k = 0; k < LGSIZE; k = k + 1) begin
			assign f_reversed_addr[k] = f_const_addr[LGSIZE-1-k];
		end
	endgenerate
	assign f_reversed_addr[LGSIZE] = f_const_addr[LGSIZE];


	always @(posedge i_clk or posedge i_reset) begin
		if (i_reset)
			f_addr_loaded <= 1'b0;
		else if (i_ce) begin
			if (wraddr == f_const_addr)
				f_addr_loaded <= 1'b1;
			else if (rdaddr == f_const_addr)
				f_addr_loaded <= 1'b0;
		end
	end


	always @(posedge i_clk)
	if (i_ce && (wraddr == f_const_addr))
		f_addr_value <= i_in;

	always @(posedge i_clk)
	if (f_past_valid && !$past(i_reset) && $past(f_addr_loaded) && !f_addr_loaded)
		`ASSERT(o_out == f_addr_value);

	always @(*)
	if (o_sync)
		`ASSERT(wraddr[LGSIZE-1:0] == 1);

	always @(*)
	if (wraddr[LGSIZE] == f_const_addr[LGSIZE] && wraddr[LGSIZE-1:0] <= f_const_addr[LGSIZE-1:0])
		`ASSERT(!f_addr_loaded);

	always @(*)
	if (rdaddr[LGSIZE] == f_const_addr[LGSIZE] && f_addr_loaded)
		`ASSERT(wraddr[LGSIZE-1:0] <= f_reversed_addr[LGSIZE-1:0] + 1);

	always @(*)
	if (f_addr_loaded)
		`ASSERT(brmem[f_const_addr] == f_addr_value);

	// Make Verilator happy
	wire unused_formal;
	assign unused_formal = &{1'b0, f_reversed_addr[LGSIZE]};
`endif
endmodule
