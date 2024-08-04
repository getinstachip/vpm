module acc (
   
    input clk,
    input rst_n,

   
    input en,
    input signed [4:0] data_in,  // I_BW = 5
    input valid_in,

    
    output signed [7:0] data_out, // O_BW = 8
    output valid_out
);

    // Instantiating the parameterized integrator
    integrator #(
        .I_BW(5),   
        .O_BW(8)    
    ) u_integrator (
        .clk_i(clk),
        .rst_n_i(rst_n),
        .en_i(en),
        .data_i(data_in),
        .valid_i(valid_in),
        .data_o(data_out),
        .valid_o(valid_out)
    );

endmodule
