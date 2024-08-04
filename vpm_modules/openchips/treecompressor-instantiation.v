module DW02_tree_inst(inst_INPUT, OUT0_inst, OUT1_inst);

parameter num_inputs = 8;
parameter input_width = 8;
parameter verif_en = 1; 

input [num_inputs*input_width-1 : 0] inst_INPUT;
output [input_width-1 : 0] OUT0_inst;
output [input_width-1 : 0] OUT1_inst;

// Instance of DW02_tree
DW02_tree #(num_inputs, input_width, verif_en) U1 (
    .INPUT(inst_INPUT),
    .OUT0(OUT0_inst),
    .OUT1(OUT1_inst)
);

endmodule
