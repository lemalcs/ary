import ary

the_string="Airport (Bush)🙂"

# Remove non printing chars
print("Original string:" + the_string)
print("Clean string: " + ary.remove_non_printing_char(the_string))

# Validate number using modulus 10 algorithm
print(ary.validate_modulus_10("5014567795"))
print(ary.validate_modulus_10("7000000001"))