**Day 4 –Practice Non –Tech** 

# **Cutting my own draft by 1/3 without losing meaning** 

**PR Title:** Fix invalid response and make registration phone number optional 

**Problem:** Registration failed when the phone number was omitted because backend validation treated it as mandatory. 

**Changes:** Removed mandatory phone number validation while retaining the valid provided number. 

**Testing: Verified** registration with a valid phone number and without one confirmed that the existing registration flow remains unaffected. 

**Impact:** Review should focus on optional phone number handling and validation of provided numbers. 

**Reflection:** This leads to users facing failure and expected behavior, then separates implementation, testing and impact so reviewers can quickly access. 

