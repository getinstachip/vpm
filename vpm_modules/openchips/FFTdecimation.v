module decimatorFFT #(parameter IWIDTH = 16, OWIDTH = IWIDTH + 1, LGWIDTH = 8, INVERSE = 0, SHIFT = 0) (
    input wire i_clk, i_reset, i_ce, i_sync,
    input wire [(2*IWIDTH-1):0] i_data,
    output reg [(2*OWIDTH-1):0] o_data,
    output reg o_sync
);

    reg wait_for_sync;
    reg [2:0] pipeline;
    reg signed [IWIDTH:0] sum_r, sum_i, diff_r, diff_i;
    reg [(2*OWIDTH-1):0] ob_a;
    wire [(2*OWIDTH-1):0] ob_b;
    reg [(OWIDTH-1):0] ob_b_r, ob_b_i;
    assign ob_b = {ob_b_r, ob_b_i};

    reg [(LGWIDTH-1):0] iaddr;
    reg [(2*IWIDTH-1):0] imem[0:1];

    wire signed [(IWIDTH-1):0] imem_r = imem[1][(2*IWIDTH-1):IWIDTH];
    wire signed [(IWIDTH-1):0] imem_i = imem[1][(IWIDTH-1):0];
    wire signed [(IWIDTH-1):0] i_data_r = i_data[(2*IWIDTH-1):IWIDTH];
    wire signed [(IWIDTH-1):0] i_data_i = i_data[(IWIDTH-1):0];

    reg [(2*OWIDTH-1):0] omem[0:1];

    wire signed [(OWIDTH-1):0] rnd_sum_r, rnd_sum_i, rnd_diff_r, rnd_diff_i;
    wire signed [(OWIDTH-1):0] n_rnd_diff_r = -rnd_diff_r;
    wire signed [(OWIDTH-1):0] n_rnd_diff_i = -rnd_diff_i;

    convround #(IWIDTH+1, OWIDTH, SHIFT) do_rnd_sum_r(i_clk, i_ce, sum_r, rnd_sum_r);
    convround #(IWIDTH+1, OWIDTH, SHIFT) do_rnd_sum_i(i_clk, i_ce, sum_i, rnd_sum_i);
    convround #(IWIDTH+1, OWIDTH, SHIFT) do_rnd_diff_r(i_clk, i_ce, diff_r, rnd_diff_r);
    convround #(IWIDTH+1, OWIDTH, SHIFT) do_rnd_diff_i(i_clk, i_ce, diff_i, rnd_diff_i);

    initial begin
        wait_for_sync = 1'b1;
        iaddr = 0;
        pipeline = 3'h0;
        o_sync = 1'b0;
    end

    always @(posedge i_clk) begin
        if (i_reset) begin
            wait_for_sync <= 1'b1;
            iaddr <= 0;
            pipeline <= 3'h0;
            o_sync <= 1'b0;
        end else if (i_ce) begin
            if (!wait_for_sync || i_sync) begin
                iaddr <= iaddr + 1'b1;
                wait_for_sync <= 1'b0;
            end
            imem[0] <= i_data;
            imem[1] <= imem[0];

            if (iaddr[1]) begin
                sum_r  <= imem_r + i_data_r;
                sum_i  <= imem_i + i_data_i;
                diff_r <= imem_r - i_data_r;
                diff_i <= imem_i - i_data_i;
            end

            pipeline <= {pipeline[1:0], iaddr[1]};

            if (pipeline[2]) o_data <= ob_a;
            else o_data <= omem[1];

            if (iaddr[1]) begin
                ob_a <= {rnd_sum_r, rnd_sum_i};
                if (!iaddr[0]) begin
                    ob_b_r <= rnd_diff_r;
                    ob_b_i <= rnd_diff_i;
                end else if (INVERSE == 0) begin
                    ob_b_r <= rnd_diff_i;
                    ob_b_i <= n_rnd_diff_r;
                end else begin
                    ob_b_r <= n_rnd_diff_i;
                    ob_b_i <= rnd_diff_r;
                end
            end

            omem[0] <= ob_b;
            omem[1] <= omem[0];
        end

        if (i_ce) o_sync <= (iaddr[2:0] == 3'b101);
    end

endmodule
