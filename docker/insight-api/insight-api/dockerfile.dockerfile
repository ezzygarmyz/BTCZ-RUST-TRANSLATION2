FROM node:16

# Set working directory
WORKDIR /app

# Copy the API source code
COPY . .

# Install dependencies
RUN npm install

# Expose API ports
EXPOSE 3001

# Start the API service
CMD ["npm", "start"]
