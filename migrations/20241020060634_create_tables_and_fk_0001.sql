CREATE TABLE users (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  phone VARCHAR(20),
  password VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  user_type VARCHAR(255) NOT NULL,
  status VARCHAR(255) NOT NULL,
  merchant_id INT
);

CREATE TABLE merchants (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  responsible_user INT,
  phone VARCHAR(20),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  merchant_type VARCHAR(255) NOT NULL,
  address VARCHAR(255),
  status VARCHAR(255) NOT NULL,
  slug VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE order_items (
  id INT AUTO_INCREMENT PRIMARY KEY,
  order_id INT NOT NULL,
  discount DECIMAL(10, 2),
  price DECIMAL(10, 2),
  product_id INT NOT NULL,
  quantity DECIMAL(10, 2) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  merchant_id INT NOT NULL,
  status VARCHAR(255) NOT NULL
);

CREATE TABLE orders (
  id INT AUTO_INCREMENT PRIMARY KEY,
  user_id INT,
  merchant_id INT NOT NULL,
  total_price DECIMAL(10, 2) NOT NULL,
  total_quantity DECIMAL(10, 2) NOT NULL,
  discount DECIMAL(10, 2),
  order_type VARCHAR(255) NOT NULL,
  status VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  transaction_id VARCHAR(255),
  payment_status VARCHAR(255) NOT NULL
);

CREATE TABLE products (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description VARCHAR(1000),
  price DECIMAL(10, 2) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  merchant_id INT NOT NULL
);

CREATE TABLE transactions (
  id INT AUTO_INCREMENT PRIMARY KEY,
  user_id INT,
  merchant_id INT NOT NULL,
  order_id INT NOT NULL,
  amount DECIMAL(10, 2) NOT NULL,
  payment_method VARCHAR(255) NOT NULL,
  status VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

ALTER TABLE order_items ADD CONSTRAINT fk_merchant_item FOREIGN KEY (merchant_id) REFERENCES merchants(id);
ALTER TABLE order_items ADD CONSTRAINT fk_product_item FOREIGN KEY (product_id) REFERENCES products(id);
ALTER TABLE order_items ADD CONSTRAINT fk_order_item FOREIGN KEY (order_id) REFERENCES orders(id);
ALTER TABLE orders ADD CONSTRAINT fk_merchant_order FOREIGN KEY (merchant_id) REFERENCES merchants(id);
ALTER TABLE products ADD CONSTRAINT fk_merchant_product FOREIGN KEY (merchant_id) REFERENCES merchants(id);
ALTER TABLE transactions ADD CONSTRAINT fk_merchant_trans FOREIGN KEY (merchant_id) REFERENCES merchants(id);
ALTER TABLE transactions ADD CONSTRAINT fk_order_trans FOREIGN KEY (order_id) REFERENCES orders(id);