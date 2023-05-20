# Treasury

This blueprint is meant to demonstrate basic functionality of a shared wallet.

Instructions:
1. `resim reset`
2. `resim new-account` -> Save the account address somewhere
3. `resim publish .` -> Make sure to be located in the "tokenize_real_estate" directory
4. `resim call-function [package_address] Treasury instantiate_treasury` -> Save the component address somewhere

RTM Commands:
`resim run component_start.rtm`
`resim run component_test_all_methods.rtm`

*Note: Be sure to fill out the variables in the .rtm files before running*