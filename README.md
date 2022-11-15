# oct-poap

The purpose of this contract is to mint NFT to proof of attendance for octopus community activities.
 
# Terminology

- `owner`: The owner of this contract. Some interfaces can only be called by owner. 
- `activity`: An activity created by `activity creator`. An activity has a token metadata and some nfts, all of nfts will use this token metadata.
- `creator`: An creator can create lots of activities, and mint nfts for participants of the activity.
- `creator whitelist`: Only owner can add or remove creators in whitelist. Only accounts in whitelist are creators.