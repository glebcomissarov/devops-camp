.SHELL: /bin/bash -e

write:
	echo "127.0.0.1   actix-app.cloud.ru" | sudo tee -a /etc/hosts

delete_on_linux:
	sudo sed -i '/127.0.0.1   actix-app.cloud.ru/d' /etc/hosts

delete_on_mac:
	sudo sed -i.bak '/127.0.0.1   actix-app.cloud.ru/d' /etc/hosts
    

