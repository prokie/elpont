# Circuit

## Node Voltage Analysis Method

To solve for the voltages at the nodes of a circuit, follow these steps:

1. **Label Nodes**: Assign labels to all nodes in the circuit.
2. **Select Common Ground**: Choose a node to be the common ground and set its voltage to zero.
3. **Handle Voltage Sources**: For each voltage source, use voltage differences to determine the voltage at another node. Ensure that the voltage source is connected to the common ground.
4. **Apply KCL**: At each remaining unknown nodal voltage, apply Kirchhoff's Current Law (KCL) to obtain an equation.
5. **Formulate Equations**: Implement the system of equations derived from KCL as a matrix expression, typically in the form `Ax = b`, where `A` is the coefficient matrix, `x` is the vector of unknown nodal voltages, and `b` is the vector of known values.
6. **Solve the System**: Solve the matrix expression to determine the values of the unknown nodal voltages.
