class CreateSurveySubscribers < ActiveRecord::Migration[5.2]
  def change
    create_table :survey_subscribers do |t|
      t.references :form, null: false
      t.string :email, null: false, limit: 255
      t.timestamps
    end
    add_index :survey_subscribers, :email
    add_index :survey_subscribers, %i[form_id email], unique: true
  end
end
