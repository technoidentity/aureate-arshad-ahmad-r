# **Day 3** 

 **PR Title:** Fix invalid response and make registration phone number optional. 

- **Problem:** 

While user submitting the registration form without phone number encountered an error because backend validation treats this field as mandatory. Phone number should be optional, but any provided must contain exactly 10 digits. 

#  **What changed** 

- Removes mandatory phone number validation. 

- Retained phone number when a provided. 

- Changed the invalid input scenario that returned HTTP 500 ro return HTTP 400. 

- **Testing** 

- Verified with valid phone number 

- Verified without phone number 

- The existing registration flow 

- **Impact** 

- Changes are in user validator and Registration service. 

- No database changes are included. 

- Review should focus on optional phone number and validation of provided numbers. 

#  **Reflection** 

- Started with user facing problem: Expalined the registartion failure and expected behaviour. 

- Kept implementation changes: List the test scenarios seperately to make it easier 

- for reviewers to see what was changed, waht was tested, whether any gaps remain. 

