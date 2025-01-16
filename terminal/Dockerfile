FROM node:18

# We have to install nodemon globally before moving into the working directory
RUN npm install -g nodemon

# Create app directory
WORKDIR /usr/src/app

# Install app dependencies
# A wildcard is used to ensure both package.json AND package-lock.json are copied
COPY package*.json ./

RUN npm install

# Bundle app source
COPY start.sh ./
RUN chmod +x start.sh

COPY . .

EXPOSE 6060

#Start App
# ENTRYPOINT [ "node", "./src/server.js" ]
CMD [ "bash", "./start.sh" ]