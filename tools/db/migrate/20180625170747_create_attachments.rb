class CreateAttachments < ActiveRecord::Migration[5.2]
  def change
    create_table :attachments do |t|
      t.references :user, null: false
      t.string :name, null: false, limit: 255
      t.string :size, null: false, limit: 8
      t.string :mime_type, null: false, limit: 64
      t.string :url, null: false, limit: 255
      t.timestamps
    end
    add_index :attachments, :name
  end
end
