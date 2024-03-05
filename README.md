# RedisAUTH Generator

This tool was written primarily to automatically generate a secure RedisAUTH token for use with Amazon Elasticache, which requires a minimum of 16 ASCII characters. 

The generator is set to 18 characters by default.

## Instructions

Root permissions are not required, so using ``sudo`` is not recommended. 

1. Clone this repositpory: ``git clone https://github.com/cloudfrl-com/RedisAUTH.git``
2. Move into the directory ``cd RedisAuth``
3. Make the file containing the script executable: chmod +x RedisAUTH.sh
4. Run the script: ``sh RedisAUTH.sh``
5. The generated AUTH token is printed in the terminal. 
